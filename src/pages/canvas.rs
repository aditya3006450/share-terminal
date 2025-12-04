use crate::components::canvas_topbar::CanvasTopBar;
use crate::services::api::{fetch_inbox, send_signal, SignalMessage};
use crate::utils::auth::get_auth_token;
use gloo::timers::callback::Interval;
use js_sys::JSON;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    HtmlVideoElement, MediaStream, MediaStreamTrack, RtcConfiguration, RtcIceCandidate,
    RtcIceCandidateInit, RtcPeerConnection, RtcPeerConnectionIceEvent, RtcSdpType,
    RtcSessionDescription, RtcSessionDescriptionInit, RtcTrackEvent,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct CanvasPageProps {
    pub id: String,
    pub dark_mode: bool,
    pub toggle_theme: Callback<()>,
}

#[function_component(CanvasPage)]
pub fn canvas_page(props: &CanvasPageProps) -> Html {
    let theme_class = if props.dark_mode { "dark-theme" } else { "" };
    let peer_connection: UseStateHandle<Option<Rc<RefCell<RtcPeerConnection>>>> =
        use_state(|| None);
    let remote_video_ref = use_node_ref();
    let navigator = use_navigator().unwrap();

    // Helper function to process signal messages
    let pc_state = peer_connection.clone();
    let navigator_for_signals = navigator.clone();
    let target_user_id = props.id.clone();

    let process_signal_message = Callback::from(move |message: SignalMessage| {
        let pc_state = pc_state.clone();
        let navigator_for_signals = navigator_for_signals.clone();
        let target_user_id = target_user_id.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let pc_rc_option = match (*pc_state).as_ref() {
                Some(pc) => pc,
                None => {
                    log::warn!("RTCPeerConnection not initialized when processing signal");
                    return;
                }
            };
            let pc = pc_rc_option.borrow();

            match message.signal_type.as_str() {
                "offer" => {
                    log::info!("Received offer: {:?}", message.payload);
                    let sdp_text = message.payload["sdp"].as_str().unwrap_or_default();
                    let mut sdp_init = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
                    sdp_init.set_sdp(sdp_text);

                    if let Err(e) = JsFuture::from(pc.set_remote_description(&sdp_init)).await {
                        log::error!("Failed to set remote description for offer: {:?}", e);
                        return;
                    }
                    log::info!("Remote description set for offer.");

                    match JsFuture::from(pc.create_answer()).await {
                        Ok(answer_js) => {
                            let answer = RtcSessionDescription::from(answer_js);
                            let sdp_answer = answer.sdp();
                            let mut sdp_answer_init =
                                RtcSessionDescriptionInit::new(RtcSdpType::Answer);
                            sdp_answer_init.set_sdp(&sdp_answer);

                            if let Err(e) =
                                JsFuture::from(pc.set_local_description(&sdp_answer_init)).await
                            {
                                log::error!("Failed to set local description for answer: {:?}", e);
                                return;
                            }
                            log::info!("Local description set for answer.");

                            if let Some(token) = get_auth_token() {
                                let mut payload_map = HashMap::new();
                                payload_map.insert("sdp".to_string(), Value::String(sdp_answer));

                                if let Err(e) = send_signal(
                                    token,
                                    target_user_id.clone(),
                                    "answer".to_string(),
                                    serde_json::to_value(payload_map).unwrap(),
                                )
                                .await
                                {
                                    log::error!("Failed to send answer: {:?}", e);
                                }
                            } else {
                                log::error!("Not authenticated to send answer.");
                                navigator_for_signals.push(&crate::router::router::Route::Login);
                            }
                        }
                        Err(e) => log::error!("Failed to create answer: {:?}", e),
                    }
                }
                "answer" => {
                    log::info!("Received answer: {:?}", message.payload);
                    let sdp_text = message.payload["sdp"].as_str().unwrap_or_default();
                    let mut sdp_init = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
                    sdp_init.set_sdp(sdp_text);

                    match JsFuture::from(pc.set_remote_description(&sdp_init)).await {
                        Ok(_) => log::info!("Remote description set for answer."),
                        Err(e) => {
                            log::error!("Failed to set remote description for answer: {:?}", e)
                        }
                    }
                }
                "ice_candidate" => {
                    log::info!("Received ICE candidate: {:?}", message.payload);
                    let candidate_json_value = message.payload["candidate"].clone();

                    let mut candidate_init = RtcIceCandidateInit::new("");
                    if let Some(candidate) = candidate_json_value["candidate"].as_str() {
                        candidate_init.set_candidate(candidate);
                    }
                    if let Some(sdp_m_line_index) = candidate_json_value["sdpMLineIndex"].as_f64() {
                        candidate_init.set_sdp_m_line_index(Some(sdp_m_line_index as u16));
                    }
                    if let Some(sdp_mid) = candidate_json_value["sdpMid"].as_str() {
                        candidate_init.set_sdp_mid(Some(sdp_mid));
                    }

                    match RtcIceCandidate::new(&candidate_init) {
                        Ok(candidate) => {
                            match JsFuture::from(
                                pc.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&candidate)),
                            )
                            .await
                            {
                                Ok(_) => log::info!("ICE candidate added."),
                                Err(e) => log::error!("Failed to add ICE candidate: {:?}", e),
                            }
                        }
                        Err(e) => log::error!("Failed to create RtcIceCandidate: {:?}", e),
                    }
                }
                _ => log::warn!("Unknown signal type: {}", message.signal_type),
            }
        });
    });

    // Polling for incoming signal messages
    use_effect_with((), {
        let user_id = props.id.clone();
        let navigator = navigator.clone();
        let process_signal_message = process_signal_message.clone();

        move |_| {
            let interval = Interval::new(1000, move || {
                let user_id = user_id.clone();
                let navigator = navigator.clone();
                let process_signal_message = process_signal_message.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(token) = get_auth_token() {
                        match fetch_inbox(token).await {
                            Ok(messages) => {
                                for message in messages {
                                    process_signal_message.emit(message);
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to fetch inbox: {:?}", e);
                                if e.contains("Not authenticated") {
                                    navigator.push(&crate::router::router::Route::Login);
                                }
                            }
                        }
                    } else {
                        log::error!("Not authenticated to fetch inbox.");
                        navigator.push(&crate::router::router::Route::Login);
                    }
                });
            });

            move || {
                drop(interval);
            }
        }
    });

    // Initialize RTCPeerConnection
    use_effect_with((), {
        let peer_connection_for_init = peer_connection.clone();
        let remote_video_ref = remote_video_ref.clone();
        let user_id = props.id.clone();
        let navigator = navigator.clone();

        move |_| {
            // Configure ICE servers
            let ice_servers = js_sys::Array::new();
            let ice_server_obj = js_sys::Object::new();
            js_sys::Reflect::set(
                &ice_server_obj,
                &JsValue::from_str("urls"),
                &JsValue::from_str("stun:stun.l.google.com:19302"),
            )
            .unwrap();
            ice_servers.push(&ice_server_obj);

            let mut config = RtcConfiguration::new();
            config.set_ice_servers(&ice_servers);

            // Create RTCPeerConnection with configuration
            if let Ok(pc) = RtcPeerConnection::new_with_configuration(&config) {
                let pc_rc = Rc::new(RefCell::new(pc));
                peer_connection_for_init.set(Some(pc_rc.clone()));

                // Handle ICE candidates
                {
                    let user_id_clone = user_id.clone();
                    let navigator_clone = navigator.clone();

                    let on_ice_candidate =
                        Closure::wrap(Box::new(move |event: RtcPeerConnectionIceEvent| {
                            if let Some(candidate) = event.candidate() {
                                let json_candidate = JSON::stringify(&candidate.to_json())
                                    .expect("Failed to stringify candidate");
                                let json_candidate_str = json_candidate
                                    .as_string()
                                    .expect("Failed to convert JsString to String");
                                let user_id_inner = user_id_clone.clone();
                                let navigator_inner = navigator_clone.clone();

                                wasm_bindgen_futures::spawn_local(async move {
                                    if let Some(token) = get_auth_token() {
                                        let mut payload: HashMap<String, serde_json::Value> =
                                            HashMap::new();
                                        payload.insert(
                                            "candidate".to_string(),
                                            serde_json::from_str(&json_candidate_str).unwrap(),
                                        );

                                        if let Err(e) = send_signal(
                                            token,
                                            user_id_inner.clone(),
                                            "ice_candidate".to_string(),
                                            serde_json::to_value(payload).unwrap(),
                                        )
                                        .await
                                        {
                                            log::error!("Failed to send ICE candidate: {:?}", e);
                                        }
                                    } else {
                                        log::error!("Not authenticated to send ICE candidate.");
                                        navigator_inner.push(&crate::router::router::Route::Login);
                                    }
                                });
                            }
                        })
                            as Box<dyn FnMut(RtcPeerConnectionIceEvent)>);

                    pc_rc
                        .borrow()
                        .set_onicecandidate(Some(on_ice_candidate.as_ref().unchecked_ref()));
                    on_ice_candidate.forget();
                }

                // Connection state changes
                {
                    let pc_for_cb = pc_rc.clone();
                    let on_connection_state_change = Closure::wrap(Box::new(move || {
                        let pc = pc_for_cb.borrow();
                        log::info!("Connection state: {:?}", pc.connection_state());
                    })
                        as Box<dyn FnMut()>);

                    pc_rc.borrow().set_onconnectionstatechange(Some(
                        on_connection_state_change.as_ref().unchecked_ref(),
                    ));
                    on_connection_state_change.forget();
                }

                // ICE gathering state changes
                {
                    let pc_for_cb = pc_rc.clone();
                    let on_ice_gathering_state_change = Closure::wrap(Box::new(move || {
                        let pc = pc_for_cb.borrow();
                        log::info!("ICE gathering state: {:?}", pc.ice_gathering_state());
                    })
                        as Box<dyn FnMut()>);

                    pc_rc.borrow().set_onicegatheringstatechange(Some(
                        on_ice_gathering_state_change.as_ref().unchecked_ref(),
                    ));
                    on_ice_gathering_state_change.forget();
                }

                // Incoming tracks
                {
                    let remote_video_ref = remote_video_ref.clone();
                    let on_track = Closure::wrap(Box::new(move |event: RtcTrackEvent| {
                        let stream_js_value = event.streams().get(0);
                        if !stream_js_value.is_undefined() && !stream_js_value.is_null() {
                            let media_stream = MediaStream::from(stream_js_value);
                            if let Some(video_element) = remote_video_ref.cast::<HtmlVideoElement>()
                            {
                                video_element.set_src_object(Some(&media_stream));
                            }
                        }
                    })
                        as Box<dyn FnMut(RtcTrackEvent)>);

                    pc_rc
                        .borrow()
                        .set_ontrack(Some(on_track.as_ref().unchecked_ref()));
                    on_track.forget();
                }
            } else {
                log::error!("Failed to create RTCPeerConnection");
            }

            // Cleanup - same closure type for all paths
            let pc_for_cleanup = peer_connection_for_init.clone();
            move || {
                if let Some(pc_rc) = (*pc_for_cleanup).as_ref() {
                    pc_rc.borrow().close();
                }
                pc_for_cleanup.set(None);
            }
        }
    });

    // Screen sharing handler
    let on_share_screen = {
        let peer_connection = peer_connection.clone();
        let target_user_id = props.id.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let window = match web_sys::window() {
                Some(w) => w,
                None => {
                    log::error!("window not available");
                    return;
                }
            };
            let nav = window.navigator();
            let media_devices = match nav.media_devices() {
                Ok(md) => md,
                Err(e) => {
                    log::error!("mediaDevices not available: {:?}", e);
                    return;
                }
            };

            // Call getDisplayMedia directly in user gesture
            let promise = match media_devices.get_display_media() {
                Ok(p) => p,
                Err(e) => {
                    log::error!("Failed to request display media: {:?}", e);
                    return;
                }
            };

            let peer_connection = peer_connection.clone();
            let target_user_id = target_user_id.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match JsFuture::from(promise).await {
                    Ok(js_value) => {
                        let media_stream = MediaStream::from(js_value);
                        log::info!("Got display media stream: {:?}", media_stream);

                        if let Some(pc_rc) = (*peer_connection).as_ref() {
                            // Add tracks to peer connection with empty streams array
                            let empty_streams = js_sys::Array::new();

                            for track in media_stream.get_tracks().iter() {
                                let media_stream_track = MediaStreamTrack::from(track);
                                // add_track requires 3 arguments: track, stream, and streams array
                                pc_rc.borrow_mut().add_track(
                                    &media_stream_track,
                                    &media_stream,
                                    &empty_streams,
                                );
                            }

                            // Create offer and send
                            let pc = pc_rc.borrow();
                            match JsFuture::from(pc.create_offer()).await {
                                Ok(offer_js) => {
                                    let offer = RtcSessionDescription::from(offer_js);
                                    let sdp_offer = offer.sdp();
                                    let mut sdp_offer_init =
                                        RtcSessionDescriptionInit::new(RtcSdpType::Offer);
                                    sdp_offer_init.set_sdp(&sdp_offer);

                                    match JsFuture::from(pc.set_local_description(&sdp_offer_init))
                                        .await
                                    {
                                        Ok(_) => {
                                            log::info!("Local description set for offer.");
                                            if let Some(token) = get_auth_token() {
                                                let mut payload_map = HashMap::new();
                                                payload_map.insert(
                                                    "sdp".to_string(),
                                                    Value::String(sdp_offer),
                                                );
                                                if let Err(e) = send_signal(
                                                    token,
                                                    target_user_id.clone(),
                                                    "offer".to_string(),
                                                    serde_json::to_value(payload_map).unwrap(),
                                                )
                                                .await
                                                {
                                                    log::error!("Failed to send offer: {:?}", e);
                                                }
                                            } else {
                                                log::error!("Not authenticated to send offer.");
                                                navigator
                                                    .push(&crate::router::router::Route::Login);
                                            }
                                        }
                                        Err(e) => log::error!(
                                            "Failed to set local description for offer: {:?}",
                                            e
                                        ),
                                    }
                                }
                                Err(e) => log::error!("Failed to create offer: {:?}", e),
                            }
                        } else {
                            log::error!("RTCPeerConnection not initialized when sharing screen.");
                        }
                    }
                    Err(e) => log::error!("Failed to get display media: {:?}", e),
                }
            });
        })
    };

    html! {
        <div class={theme_class}>
            <CanvasTopBar dark_mode={props.dark_mode} toggle_theme={props.toggle_theme.clone()} />
            <div class="canvas-container">
                <video ref={remote_video_ref} autoplay=true playsinline=true></video>
                <canvas id="canvas"></canvas>
                <button onclick={on_share_screen}>{"Share Screen"}</button>
            </div>
        </div>
    }
}

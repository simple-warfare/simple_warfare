use aeronet::io::bytes::Bytes;
use aeronet::io::{
    Session, SessionEndpoint,
    connection::{Disconnected, LocalAddr},
    server::{Closed, Server},
};
use aeronet_websocket::server::{ServerConfig, WebSocketServer, WebSocketServerPlugin};
use bevy::prelude::*;

use simple_warfare_shared::prelude::*;

use crate::statistics::ServerState;

use self::message::{ClientMessage, ClientMessageEvent, ClientMessageKind, ServerMessage};
pub mod message;

pub struct AdaptorServerPlugin;

impl Plugin for AdaptorServerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(WebSocketServerPlugin)
            .add_systems(Startup, open_server)
            .add_systems(Update, receiver_message)
            .add_observer(on_opened)
            .add_observer(on_closed)
            .add_observer(on_connecting)
            .add_observer(on_connected)
            .add_observer(on_disconnected)
            .add_observer(operation_message);
    }
}

fn server_config() -> ServerConfig {
    // let identity = Identity::self_signed(["localhost", "127.0.0.1", "::1"])
    //     .expect("all given SANs should be valid DNS names");
    ServerConfig::builder()
        .with_bind_default(25570)
        .with_no_encryption()
}

fn open_server(mut commands: Commands) {
    let config = server_config();
    commands.spawn_empty().queue(WebSocketServer::open(config));
}

fn on_closed(trigger: Trigger<Closed>) {
    panic!("server closed: {:?}", trigger.event());
}

fn on_opened(trigger: Trigger<OnAdd, Server>, servers: Query<&LocalAddr>) {
    let server = trigger.target();
    let local_addr = servers
        .get(server)
        .expect("opened server should have a binding socket `LocalAddr`");
    info!("{server} opened on {}", **local_addr);
}

fn on_connecting(trigger: Trigger<OnAdd, SessionEndpoint>, clients: Query<&ChildOf>) {
    let client = trigger.target();
    let Ok(&ChildOf(server)) = clients.get(client) else {
        return;
    };

    info!("{client} connecting to {server}");
}

fn on_connected(trigger: Trigger<OnAdd, Session>, clients: Query<&ChildOf>) {
    let client = trigger.target();
    let Ok(&ChildOf(server)) = clients.get(client) else {
        return;
    };

    info!("{client} connected to {server}");
}

fn on_disconnected(
    trigger: Trigger<Disconnected>,
    clients: Query<&ChildOf>,
    mut writer: EventWriter<AppExit>,
) {
    let client = trigger.target();
    let Ok(&ChildOf(server)) = clients.get(client) else {
        return;
    };

    match &*trigger {
        Disconnected::ByUser(reason) => {
            info!("{client} disconnected from {server} by user: {reason}");
        }
        Disconnected::ByPeer(reason) => {
            info!("{client} disconnected from {server} by peer: {reason}");
        }
        Disconnected::ByError(err) => {
            info!("{client} disconnected from {server} due to error: {err:?}");
        }
    }

    writer.write(AppExit::Success);
}

fn receiver_message(
    mut session: Single<&mut Session, With<ChildOf>>,
    mut commands: Commands,
    message_encode_kind: Res<MessageEncodeKind>,
) -> Result {
    for packet in session.recv.drain(..) {
        let msg = String::from_utf8(packet.payload.clone().into())
            .unwrap_or_else(|_| "(not UTF-8)".into());
        let message = ClientMessage::encode(*message_encode_kind, &msg)?;
        commands.trigger(ClientMessageEvent { message });
    }
    Ok(())
}

fn operation_message(
    trigger: Trigger<ClientMessageEvent>,
    mut session: Single<&mut Session, With<ChildOf>>,
    mut server_state: ResMut<NextState<ServerState>>,
    message_decode_kind: Res<MessageDecodeKind>,
) -> Result {
    let ClientMessageEvent { message } = &*trigger;
    match message.kind {
        ClientMessageKind::StartServer => server_state.set(ServerState::AssetsLoading),
        ClientMessageKind::GetServerInfo => {
            let server_info = ServerMessage::server_info();
            session
                .send
                .push(server_info.to_bytes(*message_decode_kind)?)
        }
        ClientMessageKind::CrateRoom => {}
        ClientMessageKind::ContentDecodeKind => todo!(),
        ClientMessageKind::GetMapInfos => todo!(),
        ClientMessageKind::GetMapPaths => todo!(),
    }
    Ok(())
}

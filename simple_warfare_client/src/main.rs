use aeronet::io::{
    Session, SessionEndpoint,
    connection::{Disconnected, LocalAddr},
    server::{Closed, Server},
};
use aeronet_websocket::server::{ServerConfig, WebSocketServer, WebSocketServerPlugin};
use bevy::prelude::*;

use aeronet::io::bytes::Bytes;

use simple_warfare_client::statistics::ClientState;
use simple_warfare_shared::SimpleWarfareSharedPlugin;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(SimpleWarfareSharedPlugin)
        .add_plugins(WebSocketServerPlugin)
        .add_systems(OnEnter(ClientState::StartServerWaiting), open_server)
        .add_systems(
            Update,
            receiver_client_message.run_if(in_state(ClientState::StartServerWaiting)),
        )
        .add_observer(on_opened)
        .add_observer(on_closed)
        .add_observer(on_connecting)
        .add_observer(on_connected)
        .add_observer(on_disconnected)
        .add_observer(operation_client_message)
        .run();
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

fn on_disconnected(trigger: Trigger<Disconnected>, clients: Query<&ChildOf>) {
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
}

fn receiver_client_message(
    mut clients: Query<(Entity, &mut Session), With<ChildOf>>,
    mut commands: Commands,
) -> Result {
    for (client, mut session) in &mut clients {
        // explicit deref so we can access disjoint fields
        let session = &mut *session;
        for packet in session.recv.drain(..) {
            let msg = String::from_utf8(packet.payload.clone().into())
                .unwrap_or_else(|_| "(not UTF-8)".into());
            info!("{client} > {msg}");
            let message: ClientMessage = serde_json::from_str(&msg)?;
            commands.trigger(ClientMessageEvent { client, message });
        }
    }
    Ok(())
}

fn operation_client_message(
    trigger: Trigger<ClientMessageEvent>,
    mut clients: Query<&mut Session, With<ChildOf>>,
    mut server_state: ResMut<NextState<ServerState>>,
) -> Result {
    let ClientMessageEvent { client, message } = &*trigger;
    let mut session = clients.get_mut(*client)?;
    match message.kind {
        ClientMessageKind::StartServer => server_state.set(ServerState::AssetsLoading),
        ClientMessageKind::StartLobby => todo!(),
        ClientMessageKind::GetServerInfo => {
            let server_info = ServerMessage {
                kind: ServerMessageKind::ServerInfo,
                content: Some(ServerMessageContent::ServerInfo {
                    game_version: GAME_VERSION.to_string(),
                }),
            };
            session
                .send
                .push(Bytes::from(serde_json::to_string(&server_info)?))
        }
    }
    Ok(())
}

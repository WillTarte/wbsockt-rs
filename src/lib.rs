use std::net::{TcpStream, TcpListener, ToSocketAddrs, SocketAddr};

enum WebSocketState {
    CONNECTING,
    OPEN,
    CLOSING,
    CLOSED
}

enum WebSocketEvent {
    CLOSE,
    ERROR,
    MESSAGE(),
    OPEN
}

struct WebSocketServer {
    address: SocketAddr,
    stream: TcpListener,
    sub_protocols: Vec<String>,
    extensions: Vec<String>,
    state: WebSocketState,
}

impl WebSocketServer {

    /// accept HTTP upgrade request, validate, and respond
    fn accept_upgrade(&mut self, request: todo!()) {
        todo!()
    }

    fn on_close(&mut self, request: todo!()) {
        todo!()
    }

    fn on_error(&mut self, request: todo!()) {
        todo!()
    }

    fn on_message(&mut self, request: todo!()) {
        todo!()
    }

    fn on_open(&mut self, request: todo!()) {
        todo!()
    }
}

struct WsBuilder {
    is_secure: bool,
    addr: &'static str,
    protocols: Vec<String>,
    extensions: Vec<String>
}

impl WsBuilder {

    ///create a new instance of the builder with default values
    fn new() -> Self {
        WsBuilder {
            is_secure: false,
            addr: "",
            protocols: Vec::new(),
            extensions: Vec::new()
        }
    }

    ///change is_secure value (ws vs wss)
    fn with_secure(&mut self, secure: bool) -> &mut WsBuilder {
        self.is_secure = secure;
        self
    }

    ///change socket address of server
    fn with_address<A: ToSocketAddrs>(&mut self, addr: A) -> &mut WsBuilder {
        self.addr = addr;
        self
    }

    ///add subprotocols to the WebSocketServer
    fn with_sub_protocols<A: Iterator>(&mut self, protocols: A) -> &mut WsBuilder {
        for protocol in protocols {
            self.protocols.push(protocol)
        };
        self
    }

    ///add extensions to the WebSocketServer
    fn with_extensions<A: Iterator>(&mut self, extensions: A) -> &mut WsBuilder {
        for extension in extensions {
            self.protocols.push(extension)
        };
        self
    }

    ///consume the builder and create instance of WebSocketServer
    fn build(mut self) -> WebSocketServer {
        WebSocketServer {
            address: self.addr.into(),
            stream: TcpListener::bind(self.addr.into()).unwrap(),
            sub_protocols: self.protocols,
            extensions: self.extensions,
            state: WebSocketState::CONNECTING,
        }
    }
}


#[cfg(test)]
mod tests {
    use std::net::TcpStream;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

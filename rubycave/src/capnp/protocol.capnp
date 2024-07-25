@0xed864602f086406b;

enum HandshakeResult {
    ok @0;
    versionMismatch @1;
}

interface Implementation {
    handshake @0 (version :Text) -> (result :HandshakeResult);
}

@0xf1cc09ec7e912f3d;

using import "protocol.capnp".Implementation;

interface Server extends(Implementation) {
    keepAlive @0 (epoch :UInt64);
}

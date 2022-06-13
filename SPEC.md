# Serialization Format

## Numbers

All numbers are stored BE.

## Strings

Non-`null` strings are stored as:

```
1: u8 | len: u16 | char...
```

A `null` string is simply stored as:

```
0: u8
```

# Packets

## TCP (reliable)

All TCP packets are encoded as the following:

```
len: u16 | <packet data>
```

## UDP (unreliable)

UDP packets are simply directly encoded with the packet data.

# Framework Message

[`ArcNetProvider`](https://github.com/Anuken/Mindustry/blob/8c32acbc30187e42ca0b499fdd577704879f3020/core/src/mindustry/net/ArcNetProvider.java#L410)

Framework messages are encoded as 

```
-2: i8 | <message data>
```

## DiscoverHost

This message is sent as a UDP message from the client to the server when the connection is opened.

```
1: u8
```

### Register UDP

```
3: u8 | id: u32
```

## Register TCP

```
4: u8 | id: u32
```

# Packet Message

[`ArcNetProvider.PacketSerializer`](https://github.com/Anuken/Mindustry/blob/8c32acbc30187e42ca0b499fdd577704879f3020/core/src/mindustry/net/ArcNetProvider.java#L372)

```
id: u8 | length: 16 | compressed: bool | <data>
```

- `id` is the message type

- `length` is the length of the uncompressed data

- `compressed` specifies whether compression is used. If so, the algorithm is LZ4.

[`Net::registerPacket`](https://github.com/Anuken/Mindustry/blob/8c32acbc30187e42ca0b499fdd577704879f3020/core/src/mindustry/net/Net.java#L66): The `id` is the index of the `packetProvs` array, which is determined by the order that `registerPacket` is called.

- First, `registerPacket` is called in the [`Net::static`](https://github.com/Anuken/Mindustry/blob/8c32acbc30187e42ca0b499fdd577704879f3020/core/src/mindustry/net/Net.java#L41) block. This is responsible for the first 4 packet types.

- Then, `mindustry.gen.Call::registerPackets` is called, which can be seen through a jar decompiler. It is generated through [`CallGenerator`](https://github.com/Anuken/Mindustry/blob/9aae443e7274da35f35eebcffc394ad2e9a977c9/annotations/src/main/java/mindustry/annotations/remote/CallGenerator.java#L60), which is called by [`RemoteProcess`](https://github.com/Anuken/Mindustry/blob/28b235ef07be92808cdba260168ff314db426376/annotations/src/main/java/mindustry/annotations/remote/RemoteProcess.java#L19), which is used by the [`Remote`](https://github.com/Anuken/Mindustry/blob/28b235ef07be92808cdba260168ff314db426376/annotations/src/main/java/mindustry/annotations/Annotations.java#L226) annotation.

## StreamBegin [0]

```
id: u32 | total: u32 | type: u8
```

- `id` is a counter that increases.

## StreamChunk [1]

```
id: u32 | len: u16 | data: u8...
```

## WorldStream [2]

## ConnectPacket [3]

[`Packets.ConnectPacket`](https://github.com/Anuken/Mindustry/blob/462a64bf21710fa0738f9526697dab62466e0a00/core/src/mindustry/net/Packets.java#L120) shows how it is serialised.

```
build_version: i32 | version_type: str | player_name: str | locale: str | usid: str | uuid: [16]u8 | mobile: bool | color: u32 | mods: (u8,  str...)
```

- The last half of the `uuid` is always a checksum of the first half. See the thing for more information.

---

Packet ID = Line # - 1462

## ClientSnapshot [x0D]

```
id: u32 | unit_id: u32 | dead: bool | 
pos: (f32, f32) | pointer: (f32, f32) |
rotation: f32 | base_rotation: f32 |
vel: (f32, f32) |
mining: (u16, u16) |
boosting: bool | shooting: bool | chatting: bool | building: bool |
requests: (i16, BuildPlan...) |
view: (f32, f32) | view_size: (f32, f32) |
```

- `requests[0]` is -1 if there are no requests.

A `BuildPlan` is made of either the following, for destruction:

```
breaking <true>: bool | pos: (u16, u16) |
```

Or for construction:

```
breaking <false>: bool | pos: (u16, u16) |
block_id: u16 | rotation: u8 | 1: u8 | 
config: ???
```

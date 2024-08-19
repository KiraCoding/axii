# Wild Hunt Script Extender (WHSE) [Codename: AXII]
> [!IMPORTANT]\
> WHSE is currently under development, and official releases have not been shipped yet.

| Crate         | Description   |
|---------------|-----------------------------|
| [axii_loader] | Plugin loader               |
| [axii_proxy]  | `dll` injection proxy       |
| [axii_runner] | `cargo run` dev tool        |
| [axii_sdk]    | Witcher 3 SDK Rust bindings |
| [igni]        | `dll` injection utilities   |
| [roach]       | Plugin for dev testing      |

## Compatiblity
WHSE will support the latest version of `The Witcher 3: Wild Hunt` DX11 & DX12 available on Steam.

## Installation
1. Copy `dinput8.dll` into `bin/x64_dx12`.
2. Copy `axii.dll` into `bin/whse`.
3. Place all your plugins into `bin/whse/plugins`.

## License
> [!NOTE]\
> This project is independent and not affiliated with `CD Project RED` in any way.

Todo: Decide on licensing

[axii_loader]: ./axii_loader/
[axii_proxy]:  ./axii_proxy/
[axii_runner]: ./axii_runner/
[axii_sdk]:    ./axii_sdk/
[igni]:        ./igni/
[roach]:       ./roach/

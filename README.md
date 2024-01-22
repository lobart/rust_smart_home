# Rust Smart House project

SmartHouse is project for common IOT devices. 

[![Build Status](https://github.com/lobart/rust_smart_home/workflows/Mean%22Bean%22CI/badge.svg)](https://github.com/lobart/rust_smart_home/actions?query=workflow%3A%22Mean+Bean+CI%22)
[![Build Status](https://github.com/lobart/rust_smart_home/workflows/Docker%22Image%22CI/badge.svg)](https://github.com/lobart/rust_smart_home/actions?query=workflow%3A%22Docker+Image_CI%22)
[![Build Status](https://github.com/lobart/rust_smart_home/workflows/Rust/badge.svg)](https://github.com/lobart/rust_smart_home/actions?query=workflow%3A%Rust)
[![codecov](https://codecov.io/gh/lobart/rust_smart_home/graph/badge.svg?token=67K5ZEGT4Y)](https://codecov.io/gh/lobart/rust_smart_home)

## Example of config for Smart House

```json
r#"
        {
            "name": "New House 1",
            "rooms": {
                "room1": {
                    "name": "Room 1",
                    "devices": {
                        "dev1": {
                            "name": "SmartSocket 1",
                            "typ": "SS",
                            "description": "This is SmartSocket"
                        },
                        "dev2": {
                            "name": "SmartThermometer 1",
                            "typ": "ST",
                            "description": "This is SmartThermometer"
                        }
                    }
                },
                "room2": { 
                    "name": "Room 2",
                    "devices": {
                        "dev3": {
                            "name": "SmartSocket 2",
                            "typ": "SS",
                            "description": "This is SmartSocket"
                        },
                        "dev4": {
                            "name": "SmartThermometer 2",
                            "typ": "ST",
                            "description": "This is SmartThermometer"
                        }
                    } 
                }
            }
}"#
```
## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)

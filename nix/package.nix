{ rustPlatform, openssl, pkg-config, lib }:
  rustPlatform.buildRustPackage {
    pname = "aufbaubot";
    version = (builtins.fromTOML (builtins.readFile ../Cargo.toml)).package.version;
    src = ../.;
    cargoLock.lockFile = ../Cargo.lock;
    meta = with lib; {
      description = "";
      homepage = "https://github.com/StckOverflw/aufbaubot";
      license = licenses.mit;
    };
    buildInputs = [
      openssl
    ];
    nativeBuildInputs = [
      pkg-config
    ];
  }
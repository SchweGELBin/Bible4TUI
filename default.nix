{ lib, rustPlatform }:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "bible4tui";
  version = "0.2.0";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  doInstallCheck = true;

  meta = {
    description = "Bible program for the terminal";
    homepage = "https://github.com/SchweGELBin/Bible4TUI";
    changelog = "https://github.com/SchweGELBin/Bible4TUI/blob/v${finalAttrs.version}/docs/CHANGELOG.md";
    license = lib.licenses.mit;
    mainProgram = finalAttrs.pname;
    maintainers = with lib.maintainers; [ SchweGELBin ];
  };
})

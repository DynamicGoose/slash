with import <nixpkgs> {};

stdenv.mkDerivation rec {
  name = "iced-env";
  buildInputs = [
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr libGL freetype pkg-config freetype.dev expat wayland libxkbcommon
  ];

  # WINIT_UNIX_BACKEND=wayland/x11

  LD_LIBRARY_PATH = builtins.foldl'
    (a: b: "${a}:${b}/lib") "${vulkan-loader}/lib" buildInputs;
}

# Maintainer: WMDE <https://wmde.fun>
# Contributor: System76 <jeremy@system76.com> (original cosmic-term)
# Builds our fork Lin-WMDE/wmde-term (branch wmde; master mirrors pop-os upstream).
# WMDE terminal emulator: fork of cosmic-term rebranded to fun.wmde.term, built against
# ../libcosmic (fun.wmde config) and using ../wmde-files's file dialog (compiled in, not a
# runtime package dep). build.rs/xdgen was dropped; res/ templates are installed directly.
pkgname=wmde-term
pkgver=1.5.0
pkgrel=2
pkgdesc="WMDE Terminal - terminal emulator for the WMDE desktop (fork of cosmic-term)"
arch=('x86_64')
url="https://wmde.fun"
license=('GPL-3.0-only')
# !debug: skip the split-debug package for this large statically-linked libcosmic binary.
options=('!debug')
# depends: readelf-NEEDED core libs + the libcosmic runtime set (wayland/mesa/fontconfig/
# freetype2 are dlopen'd by libcosmic at runtime). Verify with namcap.
depends=('glibc' 'gcc-libs' 'glib2' 'libxkbcommon' 'wayland' 'mesa' 'fontconfig' 'freetype2')
optdepends=('wmde-icons-base: WMDE icon theme')
# makedepends: same toolchain/libs that build the fork in Docker + glib2.
makedepends=('rust' 'cargo' 'git' 'clang' 'lld' 'pkgconf' 'glib2' 'mesa' 'wayland'
             'libxkbcommon' 'fontconfig' 'freetype2' 'expat' 'zstd')
source=("$pkgname::git+https://github.com/Lin-WMDE/wmde-term.git#branch=wmde")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  # WMDE unified version: 1.5 (libcosmic base) . <commits since nearest tag> . g<short>.
  local desc
  desc=$(git describe --long --tags --abbrev=7 2>/dev/null || true)
  if [ -n "$desc" ]; then
    printf '1.7.%s.g%s' "$(printf '%s' "$desc" | sed -E 's/.*-([0-9]+)-g[0-9a-f]+$/\1/')" "$(git rev-parse --short=7 HEAD)"
  else
    printf '1.7.%s.g%s' "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
  fi
}

build() {
  cd "$srcdir/$pkgname"
  # x86-64-v3 (AVX2/BMI2) baseline for the WMDE repo; runs on Haswell+ (and the VM).
  export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C target-cpu=x86-64-v3"
  # link the system libzstd; the vendored zstd-sys build drops symbols under lld
  export ZSTD_SYS_USE_PKG_CONFIG=1
  cargo build --release --bin wmde-term
}

package() {
  cd "$srcdir/$pkgname"
  local appid=fun.wmde.term
  install -Dm755 "${CARGO_TARGET_DIR:-target}/release/wmde-term" "$pkgdir/usr/bin/wmde-term"
  install -Dm644 "res/$appid.desktop" "$pkgdir/usr/share/applications/$appid.desktop"
  # The key bindings the terminal handles itself, so that Settings can offer a page
  # for them without knowing anything about terminals.
  install -Dm644 "res/app-shortcuts/$appid.ron" \
    "$pkgdir/usr/share/wmde/app-shortcuts/$appid.ron"
  install -Dm644 "res/$appid.metainfo.xml" "$pkgdir/usr/share/metainfo/$appid.metainfo.xml"
  # App icons (all sizes) for the icon theme lookup.
  for icon in res/icons/hicolor/*/apps/"$appid".svg; do
    local size
    size=$(basename "$(dirname "$(dirname "$icon")")")
    install -Dm644 "$icon" "$pkgdir/usr/share/icons/hicolor/$size/apps/$appid.svg"
  done
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

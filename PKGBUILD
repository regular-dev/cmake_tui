# Maintainer: regular-dev.org <regular-dev@gmail.com>
pkgname=cmake_tui
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
url='regular-dev.org'

build() {
    return 0
}

package() {
    cargo install --root="$pkgdir/usr/local" --path "../"
    rm "$pkgdir/usr/local/.crates.toml"
    rm "$pkgdir/usr/local/.crates2.json"
}

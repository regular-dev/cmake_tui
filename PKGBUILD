# Maintainer: Xxxion <xionovermazes@yahoo.com>
pkgname=cmake_tui
pkgver=1.0.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="CMake configuration via terminal user interface"
license=('MIT')

build() {
    return 0
}

package() {
    cargo install --root="$pkgdir" cmake_tui
}

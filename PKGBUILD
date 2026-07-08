# Maintainer: selcom slepanskiy
pkgname=apm
pkgver=1.0.0
pkgrel=1
pkgdesc="A simple universal wrapper for package managers"
arch=('x86_64')
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo')

build() {
    cargo build --release --locked
}

package() {
    install -Dm755 target/release/apm "$pkgdir/usr/bin/apm"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

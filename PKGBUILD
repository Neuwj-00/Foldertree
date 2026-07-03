# Maintainer: Neuwj <neuwj@linuxmail.org>
pkgname=ftr
_realname=Foldertree
pkgver=1.0.0
pkgrel=1
pkgdesc="Generates a directory tree from the current folder and can copy the result to the clipboard"
arch=('x86_64')
url="https://github.com/Neuwj-00/Foldertree"
license=('GPL3')
depends=('xclip' 'wl-clipboard' 'gcc-libs')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$_realname-$pkgver"
  cargo build --release --locked --all-features --target-dir=target
}

package() {
  cd "$_realname-$pkgver"
  
  
  install -Dm 755 "target/release/foldertree" "$pkgdir/usr/bin/$pkgname"
  
  install -Dm 644 README.md -t "$pkgdir/usr/share/doc/$pkgname"
  install -Dm 644 LICENSE -t "$pkgdir/usr/share/licenses/$pkgname"
}
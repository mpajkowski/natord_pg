pkgname=natord-pg-git
_pkgname=natord_pg
pkgver=0.1.0
pkgrel=1
pkgdesc='Natural ordering operators for PostgreSQL text type'
arch=('x86_64')
url='https://github.com/mpajkowski/natord_pg'
license=('custom')
depends=('gcc-libs' 'postgresql')
makedepends=('git' 'cargo' 'cargo-pgx')
source=("$pkgname::git+$url.git")
b2sums=('SKIP')

# pkgver() {
#   cd "$pkgname"
#
#   git describe --tags | sed 's/^v//'
# }

prepare() {
  cd "$pkgname"

  # download dependencies
  cargo update
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
  cargo pgx init --pg14 /usr/bin/pg_config
}

build() {
  cd "$pkgname"
  cargo pgx package --no-default-features --features pg14
}

package() {
  cd "$pkgname"

  install -vDm644 "target/release/$_pkgname-pg14/usr/share/postgresql/extension/natord_pg.control"  "$pkgdir/usr/share/postgresql/extension/natord_pg.control"
  install -vDm644 "target/release/$_pkgname-pg14/usr/share/postgresql/extension/natord_pg--0.0.0.sql"  "$pkgdir/usr/share/postgresql/extension/natord_pg--0.0.0.sql"
  install -vDm755 "target/release/$_pkgname-pg14/usr/lib/postgresql/natord_pg.so" "$pkgdir/usr/lib/postgresql/natord_pg.so"

  # documentation
  install -vDm644 README.md "$pkgdir/usr/share/doc/$pkgname" 
}

thisdir=$(dirname $BASH_SOURCE)
dir=$1
[ ! "$dir" ] && dir=.

mdfiles="$(find "$dir" -maxdepth 1 | grep \.md$)"

# cargo run $mdfiles
if [ ! -s "$thisdir/../target/debug/markdown" ] ; then
  cargo build
fi

$thisdir/../target/debug/markdown $mdfiles

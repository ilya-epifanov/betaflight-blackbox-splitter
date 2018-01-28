# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    local suffix=""
    if [[ $TARGET =~ .*-windows-.* ]]; then
        suffix=".exe"
    fi

    cross rustc --bin betaflight-blackbox-splitter --target $TARGET --release -- -C lto

    cp target/$TARGET/release/betaflight-blackbox-splitter$suffix $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main

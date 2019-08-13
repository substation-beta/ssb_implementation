:: Build release version of project
cargo build --release
:: Archive output
pushd target\release
7z a ..\..\ssb_filter.zip ssb_filter.dll ssb_filter.dll.lib ssb_filter.h
popd
:: Push archive to AppVeyor artifacts
appveyor PushArtifact ssb_filter.zip -DeploymentName "SSB filter"
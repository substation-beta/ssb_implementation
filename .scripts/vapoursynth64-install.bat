:: PYTHON
:: Select pre-installed python version
set PYTHON=C:\Python37-x64
:: Add global access to python binaries (overwrite default python 2.7)
set PATH=%PYTHON%;%PATH%
:: Show python version (detailed)
python -VV
:: VAPOURSYNTH
:: Load vapoursynth R47
curl -fsSL -o VapourSynth64-Portable-R47.7z https://github.com/vapoursynth/vapoursynth/releases/download/R47/VapourSynth64-Portable-R47.7z
:: Extract vapoursynth archive into python
7z x VapourSynth64-Portable-R47.7z -o%PYTHON% -y
:: Show vapoursynth version
python -c "import vapoursynth; print(vapoursynth.core.version())"
:: Add compiler access to vapoursynth sdk
set VAPOURSYNTH_LIB_DIR=%PYTHON%/sdk/lib64
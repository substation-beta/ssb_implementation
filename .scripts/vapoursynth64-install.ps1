# Get python home
$Env:PYTHON = split-path (Get-Command python).Source
# Load vapoursynth R52
Invoke-WebRequest https://github.com/vapoursynth/vapoursynth/releases/download/R52/VapourSynth64-Portable-R52.7z -OutFile "VapourSynth64-Portable-R52.7z"
# Extract vapoursynth archive into python
7z x VapourSynth64-Portable-R52.7z -o"$Env:PYTHON" -y '-xr!7z.*'
# Show vapoursynth version
python -c "import vapoursynth; print(vapoursynth.core.version())"
# Add compiler access to vapoursynth sdk
echo "VAPOURSYNTH_LIB_DIR=$Env:PYTHON/sdk/lib64" | Out-File -FilePath $Env:GITHUB_ENV -Append -Encoding utf8    # https://help.github.com/en/actions/reference/workflow-commands-for-github-actions#setting-an-environment-variable
# Cleanup
Remove-Item VapourSynth64-Portable-R52.7z

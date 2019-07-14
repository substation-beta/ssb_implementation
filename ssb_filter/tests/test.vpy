# Import vapoursynth core
from vapoursynth import core

# Load SSB plugin (inserted by post-processor)
core.std.LoadPlugin({:?})

# Create empty clip
clip = core.std.BlankClip(None, 100, 100)
# Apply SSB rendering
clip = core.ssb.render_raw(clip, '#EVENTS\n0-1.|||')

# Trigger some processing
clip.get_frame(0)
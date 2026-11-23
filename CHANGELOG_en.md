# Changelog

All notable changes to Raster Tools are recorded here.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning: [Semantic Versioning](https://semver.org/) —
`0.x.y` means an initial version, still being tested.

This is the English copy of `CHANGELOG.md` (the Italian original, which the GitHub
release notes are taken from). `scripts/release.ps1` moves the entries under
**[Unreleased]** into the new version when a release is made.

## [Unreleased]

## [0.11.0] - 2026-11-23
### Changed
- Rebuilt for Open CAD Studio v2026.47 (plugin API 28)


## [0.10.3] - 2026-11-16

### Added
- `RT_OSM`: when the drawing has a geographic location (the `GEOGRAPHICLOCATION` command, GEODATA object),
  its EPSG code is read and filled into the dialog's EPSG field (it takes priority over the last-used
  code). With no geographic location the last-used code is kept.

### Changed
- Rebuilt for Open CAD Studio v2026.46 (plugin API 23): fixes the load error
  "plugin reads codec types of wire schema …, but the host writes …".

## [0.10.2] - 2026-11-14

### Added
- `RT_OSM` with nothing selected: asks for the lower-left and the upper-right corner
  (live rectangle and size); in the dialog, the "Re-pick the two corners" checkbox asks for them again.
- Drop-down list of sources: OpenStreetMap, Esri World Imagery, USGS Imagery, Custom (URL).
- "Show the Esri terms of use" checkbox (text shown on the command line).
- Esri World Imagery: an MTEXT with the date, resolution, accuracy and provider of the imagery
  (the service's layer 4, "Citations"), placed next to the image. Tiles from Esri are not cached.

### Fixed
- `RT_OSM`: the command was cancelled after the second corner ("Command cancelled"): the Enter
  that the CAD appends to a multi-word command line closed the dialog at once. The arguments now go
  through the "deferred" mechanism (also for "Re-pick" and "Show the terms").

## [0.10.1] - 2026-11-14

### Fixed
- `RT_OSM`: the tile cache is decoded by content, so JPEG/WebP tiles (typical of aerial
  imagery) work even when stored with a .png extension.
- `RT_OSM`: with a URL other than OpenStreetMap the layer is `BASEMAP`, the file is named
  `map_epsg…` and the credit text is "Map tiles: <host>" instead of "© OpenStreetMap contributors".

## [0.10.0] - 2026-11-13

### Added
- **Base map** (`RT_OSM`, Insert tab): select a rectangle (closed polyline), give the
  drawing's EPSG code (UTM, NZTM, etc.) and the plugin downloads the OpenStreetMap tiles,
  reprojects them into a north-up raster in the drawing's coordinate system and inserts it
  (PNG + world file beside the drawing, layer `OSM`, behind the drawing) with the credit
  "© OpenStreetMap contributors". Automatic or chosen pixel size, tile limit
  (64 by default), configurable tile source, 7-day cache.
  Quick form: `RT_OSM <EPSG> [pixel] [max-tiles]`.

## [0.9.4] - 2026-11-13

### Added
- **Alpha on** (`RT_ALPHAON`, Transparency tab): turns the transparency flag on for the
  selected images, or for all images in the drawing when nothing is selected, so the alpha
  channel of 32-bit PNG/TIFF files is used. Only the flag changes: no file is written
  or copied. `RT_TRANSPOFF` turns it off.

## [0.9.3] - 2026-11-13

### Changed
- Images inserted with `RT_ATTACH` and `RT_FOLDER` now have **transparency
  on**: the alpha channel of 32-bit PNG/TIFF files is respected on screen and in
  prints/PDFs (before, transparent areas were drawn opaque, usually black).
  For images already in the drawing use `RT_TRANSP`; `RT_TRANSPOFF` turns it off.

## [0.9.2] - 2026-11-13

### Added
- **GeoTIFF**: for `.tif`/`.tiff` files the georeferencing is read from the tags in the
  file header (pixel scale + tie point, or transformation matrix; classic TIFF and
  BigTIFF; PixelIsArea/PixelIsPoint). It applies to the
  *World file* mode of `RT_ATTACH`, to `RT_WORLD` and to `RT_FOLDER`. The `.tfw` beside the file is only needed
  if the TIFF has no geographic tags. Multiple control points (GCPs) are not supported.

## [0.9.1] - 2026-11-13

### Added
- **Insert folder** (`RT_FOLDER`, Insert tab): pick any image in a folder and all the
  images in that folder are inserted, each where its world file says, on the RASTER layer
  (created if missing), behind the drawing; then zoom extents. Images without a world file and those
  already in the drawing are skipped and reported. A single undo step.

## [0.9.0] - 2026-11-13

### Added
- **World files** (`.pgw`, `.tfw`, `.jgw`, `.gfw`, `.bpw`, `.wld`): in **Insert
  image** (`RT_ATTACH`) a new size mode, *World file*: the image goes
  straight where the file beside it says (position, scale and any rotation), on the
  chosen layer, without clicks. The file is looked up in the same folder with the same
  name (case does not matter).
- **From world file** (`RT_WORLD`, Position tab): puts images already in the drawing
  (selected, or clicked) back in place by reading the world file; clips are kept.

## [0.8.0] - 2026-11-11

### Added
- **Align 2 points** (`RT_ALIGN`): georeferences an image. Click a recognisable point on
  the image and where it must go, then a second point and its destination:
  the image moves, rotates and scales by itself. Enter after the first
  pair = move only.
- **Rotate** (`RT_ROTATE`): with the mouse (centre, reference point, new
  direction: aligns to a line) or by an exact angle (Enter).
- **Inserting a PDF** (`RT_ATTACH`): from a PDF, for example a cadastral map
  extract, the plugin extracts the image it contains at its resolution and saves it
  as `<name>_pdf.png`. New **Map scale 1:** field for the DPI mode: an
  extract at 300 DPI at scale 1:2000 comes in at its real size in metres.
- **Send behind** (`RT_BACK`): images go behind the drawing's lines; images
  inserted with `RT_ATTACH` go there by themselves.
- **Clean up copies** (`RT_PURGE`): moves to a `_trasp-non-usate` subfolder the
  transparent copies that no image in the drawing uses any more.
- **Guide** (`RT_GUIDE`): a button with a short guide to all the commands.

### Changed
- The diagnostic log is now **off by default**: `RT_DIAG ON` / `RT_DIAG OFF`.

## [0.7.0] - 2026-11-09

### Added
- **Colour the map with the layer's colour** (`RT_TRANSP`). The
  "Transparent colour" dialog has a new option, *Colour the rest with the layer's
  colour*: the background disappears and everything else takes the colour of the image's
  layer, with soft edges. The black lines of a cadastral map
  become, for example, white and easy to read on a dark background, without the light halo
  and "jagged" lines that transparency alone left. It is on by default
  when the chosen colour is light (the paper of a scan).
- **Layer of the inserted image** (`RT_ATTACH`). The insert dialog asks for the
  layer: **RASTER** is proposed, created (white) if it does not exist in the drawing,
  or another layer of your choice.

## [0.6.2] - 2026-11-08

### Added
- Diagnostic log `rastertools-log.txt` in Open CAD Studio's settings
  folder: the commands received by the plugin and the steps of `RT_TRANSP`, to
  see where a command stops when the command line shows nothing.

## [0.6.1] - 2026-11-08

### Fixed
- **The "Make transparent" button did nothing.** The dialog opened, but
  the CAD closed it at once "behind the scenes" and the button had no effect.
  The same flaw could also block other steps opened after choosing the
  image: "Insert image" (picking the point) and "Size / DPI" or
  "Change path" without an image selected beforehand.

## [0.6.0] - 2026-11-08

### Changed
- **Compatible with Open CAD Studio 2026.45** (Plugin API 16). 2026.45 rejects
  plugins built for the previous version (at start-up it reported an error about
  plugins). This version works **only with 2026.45**: if you still use
  2026.44, stay on 0.5.1.

## [0.5.1] - 2026-11-08

### Fixed
- **Transparent colour on large scans.** With very large images (for example
  a 7000 × 10000 pixel cadastral map) the command stopped without any
  message: saving the PNG copy took longer than the 30 seconds Open CAD Studio
  allows a plugin command. Saving is now much faster, the plugin
  keeps the CAD informed while it works and shows a "please wait…" message for large
  images; it also uses less memory.
- If the same image is inserted several times in the drawing (overlapping copies),
  `RT_TRANSP` relinks all the copies of the same file to the transparent copy:
  before, a copy that was still opaque could cover the one made transparent.

## [0.5.0] - 2026-11-08

### Added
- **Transparent colour** (`RT_TRANSP`, Raster › Transparency tab): choose
  the image, then click **directly on the image** the colour to make
  transparent (for example the white background of a scan). A dialog shows the
  chosen colour and asks for the **tolerance** (for similar colours; 10% suggested for
  JPG, 2% for other formats) and whether to act on the whole image or **only on the area
  connected to the clicked point**. The plugin saves a PNG copy with the transparency
  beside the original (`<name>_trasp-N.png`, the original is not touched),
  relinks the image to the copy and turns transparency on. The command can be
  repeated to remove other colours.
- `RT_TRANSPOFF` (Transparency off): turns the images' transparency off.

## [0.4.0] - 2026-11-05

### Added
- **English and Italian.** The plugin speaks English, or Italian when Open CAD
  Studio is in Italian (with the "System" setting, when Windows is in Italian).
  Messages, prompts, dialogs and ribbon labels follow the language; the new
  command `RT_LANG IT|EN|AUTO` forces it by hand.
- README and command reference in English (`README.md`, `PLUGIN.md`) and in
  Italian (`README.it.md`, `PLUGIN.it.md`), with the "hobby project" notice.

## [0.3.1] - 2026-11-05

### Fixed
- The build on GitHub no longer worked: the DWG library `acadrust`, which
  Open CAD Studio depends on, was moved to another repository. The plugin now
  downloads it from the new address (same version, no change in features).
  This version also publishes the image insertion of 0.3.0.

## [0.3.0] - 2026-11-05

### Added
- **Inserting images** into the drawing (`RT_ATTACH`, *Insert image* button):
  choose the file (GIF, JPG, PNG, BMP, TIFF, WebP) and the size — given on
  screen with two points (proportional), by width, by DPI resolution or
  1 pixel = 1 unit — then click the insertion point. The image goes on the
  current layer and the file stays linked; the unit for DPI is suggested from the
  drawing's units.

## [0.2.0] - 2026-11-04

### Added
- Clipping, holing and splitting with a **polygon** drawn by clicks (`RT_CLIPPOLY`,
  `RT_HOLEPOLY`, `RT_CUTPOLY`): Enter or a click on the first vertex closes the polygon.
- Clipping, holing and splitting with a **closed polyline**, a circle or an ellipse
  already in the drawing (`RT_CLIPPL`, `RT_HOLEPL`, `RT_CUTPL`); the arc sides of
  polylines are followed faithfully. Choose the polyline by clicking it or by
  clicking inside it; if the image and the polyline are already selected the clip
  is immediate.
- In the Raster tab the Clip, Hole and Split piece buttons become menus with
  three choices: Rectangle, Polygon, From polyline.

### Changed
- A new clip on an already clipped image narrows the clip also with
  polygons (if both outlines are concave, the new one replaces the old one).
- Outlines that intersect themselves are rejected with a clear message.

## [0.1.0] - 2026-11-04

First public version, being tested.

### Added
- Rectangular clip that keeps the inside (`RT_CLIP`); repeated, it narrows the clip.
- Rectangular hole that hides the inside (`RT_CLIPHOLE`).
- Splitting a piece off into a new movable image (`RT_CUT`).
- Removing the clip (`RT_UNCLIP`).
- Proportional resizing by dragging a corner (`RT_RESIZE`).
- Resizing by width, height, scale factor or DPI resolution (`RT_SIZE`).
- Changing the file on disk, keeping the width in the drawing (`RT_PATH`).
- Searching a folder for moved files (`RT_FIND`).
- List of images with path, pixels, size and file status (`RT_LIST`).
- Formats: GIF, JPG, PNG, BMP, TIFF, WebP.
- Compatible with Open CAD Studio v2026.44 (Plugin API 14).

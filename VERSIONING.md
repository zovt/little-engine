# Versioning
## Pre-Major-Release-Disclaimer
Until the first major version is released, expect drastic API changes
frequently and without warning.

## Version Scheme
NAME MAJOR.REVISION.FEATURE.PATCH

### Major Release
Incremented when a new major release is cut. No guarantee is made with
regard to backwards compatibility. Expect MAJOR, BREAKING API changes.

A new "code name" will be assigned to each major release bump.

When a new major release is made, the other numbers will reset to 0.

### Revision Version
Incremented when NON-MAJOR, BACKWARDS-INCOMPATIBLE changes are made.
Expect MINOR, BREAKING changes to existing APIs.

When a new REVISION is cut, expect that any new FEATURE versions
WILL NOT be backported. However, they may be in some cases.

### Feature Version
Incremented when SIGNIFICANT features are added to an existing
release. Expect NO breaking changes.

### Patch Version
Incremented when MINOR, NON-BREAKING changes are made. Expect bug
fixes and NO breaking changes.

### Notes
Note that REVISION, FEATURE, and PATCH WILL NOT NECESSARILY CHANGE
when the value(s) to the left of them change. This means that you can
always choose the version of a major release with the latest features
by picking the version with the highest FEATURE version. In fact, if
you are starting a new project, you should pick this version. The
PATCH version can be used to check that your version has all of the
latest bug-fixes as well.

# Shared Metadata {#shared-metadata}

Shared metadata is defined once for the whole project and is available to
every markdown project. It lives under `[shared_metadata]` in the manifest
file and is merged with project specific metadata at conversion time.
Values defined in a markdown project override entries from the shared metadata.

Use shared metadata for information that stays the same across multiple books
or documents, like the publisher or an overarching author list.

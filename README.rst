rust-vdpau-sys
====================

:Date: 11/13 2018


.. contents::



Nvidia VDPAU ( Video Decode and Presentation API for Unix )
---------------------------------------------------------------------

*   `Wiki <https://en.wikipedia.org/wiki/VDPAU>`_
*   `VDPAU C Headers <http://http.download.nvidia.com/XFree86/vdpau/>`_
*   `VDPAU C API <https://http.download.nvidia.com/XFree86/vdpau/doxygen/html/>`_
*   `libvdpau source code <https://www.freedesktop.org/wiki/Software/VDPAU/>`_
*   `Launchpad Nvidia libvdpau <https://launchpad.net/~nvidia-vdpau/+archive/ubuntu/ppa/+sourcepub/1035482/+listing-archive-extra>`_
*   `Github NVIDIA/vdpau-hevc-example <https://github.com/NVIDIA/vdpau-hevc-example>`_
*   `Github NVIDIA/vdpau-win-x11 <https://github.com/NVIDIA/vdpau-win-x11>`_


Rust Bindgen
------------------

.. code:: bash

    bindgen --rustfmt-bindings \
        --no-layout-tests \
        --generate-inline-functions \
        --disable-name-namespacing \
        --no-prepend-enum-name \
        --link "libvdpau" \
        include/vdpau.h > src/vdpau.rs



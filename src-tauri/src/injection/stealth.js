// Advanced Stealth Script (Linux/Chrome 131 Spoofing)
(function() {
    // Helper to overwrite read-only properties
    function mockReadonly(target, prop, value) {
        Object.defineProperty(target, prop, {
            get: () => value,
            configurable: true,
            enumerable: true
        });
    }

    // 1. Client Hints (Chrome 131 on Linux)
    if (!navigator.userAgentData) {
        const userAgentData = {
            brands: [
                { brand: "Chromium", version: "131" },
                { brand: "Google Chrome", version: "131" },
                { brand: "Not_A Brand", version: "24" }
            ],
            mobile: false,
            platform: "Linux",
            getHighEntropyValues: function(hints) {
                return Promise.resolve({
                    architecture: "x86",
                    bitness: "64",
                    brands: this.brands,
                    mobile: this.mobile,
                    model: "",
                    platform: this.platform,
                    platformVersion: "6.5.0", // Example Linux kernel version
                    uaFullVersion: "131.0.6778.85",
                    fullVersionList: [
                        { brand: "Chromium", version: "131.0.6778.85" },
                        { brand: "Google Chrome", version: "131.0.6778.85" },
                        { brand: "Not_A Brand", version: "24.0.0.0" }
                    ]
                });
            }
        };
        mockReadonly(navigator, 'userAgentData', userAgentData);
    }

    // 2. Vendor Spoofing
    mockReadonly(navigator, 'vendor', 'Google Inc.');

    // 3. ProductSub Spoofing (Chrome standard)
    mockReadonly(navigator, 'productSub', '20030107');

    // 4. MaxTouchPoints (Desktop = 0)
    mockReadonly(navigator, 'maxTouchPoints', 0);

    // 5. Hardware Concurrency (Ensure realistic core count)
    if (navigator.hardwareConcurrency === undefined || navigator.hardwareConcurrency === 0) {
        mockReadonly(navigator, 'hardwareConcurrency', 4);
    }

    // 6. Canvas Fingerprinting Noise
    const originalToDataURL = HTMLCanvasElement.prototype.toDataURL;
    HTMLCanvasElement.prototype.toDataURL = function(type) {
        const context = this.getContext('2d');
        if (context) {
            const shift = {
                'r': Math.floor(Math.random() * 10) - 5,
                'g': Math.floor(Math.random() * 10) - 5,
                'b': Math.floor(Math.random() * 10) - 5,
                'a': Math.floor(Math.random() * 10) - 5
            };
            const width = this.width;
            const height = this.height;
            const imageData = context.getImageData(0, 0, width, height);
            for (let i = 0; i < height; i++) {
                for (let j = 0; j < width; j++) {
                    const n = i * (width * 4) + j * 4;
                    imageData.data[n + 0] = imageData.data[n + 0] + shift.r;
                    imageData.data[n + 1] = imageData.data[n + 1] + shift.g;
                    imageData.data[n + 2] = imageData.data[n + 2] + shift.b;
                    imageData.data[n + 3] = imageData.data[n + 3] + shift.a;
                }
            }
            context.putImageData(imageData, 0, 0);
        }
        return originalToDataURL.apply(this, arguments);
    };

    // 7. Mask navigator.webdriver
    mockReadonly(navigator, 'webdriver', undefined);

    // 8. Mock window.chrome
    if (!window.chrome) {
        window.chrome = {
            runtime: {
                OnInstalledReason: {
                    CHROME_UPDATE: "chrome_update",
                    INSTALL: "install",
                    SHARED_MODULE_UPDATE: "shared_module_update",
                    UPDATE: "update"
                },
                OnRestartRequiredReason: {
                    APP_UPDATE: "app_update",
                    OS_UPDATE: "os_update",
                    PERIODIC: "periodic"
                },
                PlatformArch: {
                    ARM: "arm",
                    ARM64: "arm64",
                    MIPS: "mips",
                    MIPS64: "mips64",
                    X86_32: "x86-32",
                    X86_64: "x86-64"
                },
                PlatformNaclArch: {
                    ARM: "arm",
                    MIPS: "mips",
                    MIPS64: "mips64",
                    X86_32: "x86-32",
                    X86_64: "x86-64"
                },
                PlatformOs: {
                    ANDROID: "android",
                    CROS: "cros",
                    LINUX: "linux",
                    MAC: "mac",
                    OPENBSD: "openbsd",
                    WIN: "win"
                },
                RequestUpdateCheckStatus: {
                    NO_UPDATE: "no_update",
                    THROTTLED: "throttled",
                    UPDATE_AVAILABLE: "update_available"
                },
                connect: function() {},
                sendMessage: function() {}
            },
            loadTimes: function() {},
            csi: function() {},
            app: {
                isInstalled: false,
                getDetails: function() {},
                getIsInstalled: function() {},
                installState: function() {},
                runningState: function() {},
                InstallState: {
                    DISABLED: "disabled",
                    INSTALLED: "installed",
                    NOT_INSTALLED: "not_installed"
                },
                RunningState: {
                    CANNOT_RUN: "cannot_run",
                    READY_TO_RUN: "ready_to_run",
                    RUNNING: "running"
                }
            }
        };
    }

    // 9. Mock Permissions API
    if (navigator.permissions) {
        const originalQuery = navigator.permissions.query;
        navigator.permissions.query = function(parameters) {
            return parameters.name === 'notifications'
                ? Promise.resolve({ state: 'granted', onchange: null })
                : originalQuery(parameters);
        };
    }

    // 10. Mock Plugins and MimeTypes
    const pluginsData = [
        {
            name: 'PDF Viewer',
            filename: 'internal-pdf-viewer',
            description: 'Portable Document Format',
            mimeTypes: [{ type: 'application/pdf', suffixes: 'pdf', description: 'Portable Document Format' }]
        },
        {
            name: 'Chrome PDF Viewer',
            filename: 'internal-pdf-viewer',
            description: 'Portable Document Format',
            mimeTypes: [{ type: 'application/pdf', suffixes: 'pdf', description: 'Portable Document Format' }]
        },
        {
            name: 'Chromium PDF Viewer',
            filename: 'internal-pdf-viewer',
            description: 'Portable Document Format',
            mimeTypes: [{ type: 'application/pdf', suffixes: 'pdf', description: 'Portable Document Format' }]
        },
        {
            name: 'Microsoft Edge PDF Viewer',
            filename: 'internal-pdf-viewer',
            description: 'Portable Document Format',
            mimeTypes: [{ type: 'application/pdf', suffixes: 'pdf', description: 'Portable Document Format' }]
        },
        {
            name: 'WebKit built-in PDF',
            filename: 'internal-pdf-viewer',
            description: 'Portable Document Format',
            mimeTypes: [{ type: 'application/pdf', suffixes: 'pdf', description: 'Portable Document Format' }]
        }
    ];

    function generatePluginArray(pluginsData) {
        const pluginArray = [];
        pluginsData.forEach(p => {
            const plugin = {
                name: p.name,
                filename: p.filename,
                description: p.description,
                length: p.mimeTypes.length,
                item: function(index) { return this[index]; },
                namedItem: function(name) { return this[name]; }
            };
            
            p.mimeTypes.forEach((m, i) => {
                const mimeType = {
                    type: m.type,
                    suffixes: m.suffixes,
                    description: m.description,
                    enabledPlugin: plugin
                };
                plugin[i] = mimeType;
                plugin[m.type] = mimeType;
            });
            
            pluginArray.push(plugin);
            pluginArray[p.name] = plugin;
        });
        
        pluginArray.item = function(index) { return this[index]; };
        pluginArray.namedItem = function(name) { return this[name]; };
        pluginArray.refresh = function() {};
        
        return pluginArray;
    }

    const fakePlugins = generatePluginArray(pluginsData);
    mockReadonly(navigator, 'plugins', fakePlugins);

    function generateMimeTypeArray(pluginsData) {
        const mimeTypeArray = [];
        pluginsData.forEach(p => {
            p.mimeTypes.forEach(m => {
                const mimeType = {
                    type: m.type,
                    suffixes: m.suffixes,
                    description: m.description,
                    enabledPlugin: navigator.plugins[p.name]
                };
                mimeTypeArray.push(mimeType);
                mimeTypeArray[m.type] = mimeType;
            });
        });
        
        mimeTypeArray.item = function(index) { return this[index]; };
        mimeTypeArray.namedItem = function(name) { return this[name]; };
        
        return mimeTypeArray;
    }

    const fakeMimeTypes = generateMimeTypeArray(pluginsData);
    mockReadonly(navigator, 'mimeTypes', fakeMimeTypes);

    // 11. WebGL Vendor/Renderer masking
    try {
        const getParameter = WebGLRenderingContext.prototype.getParameter;
        WebGLRenderingContext.prototype.getParameter = function(parameter) {
            // UNMASKED_VENDOR_WEBGL
            if (parameter === 37445) {
                return 'Intel Open Source Technology Center';
            }
            // UNMASKED_RENDERER_WEBGL
            if (parameter === 37446) {
                return 'Mesa Intel(R) UHD Graphics 620 (Kabylake GT2)';
            }
            return getParameter(parameter);
        };
    } catch (e) {
        console.error('Failed to mask WebGL', e);
    }

    // 12. Bind outer dimensions
    try {
        if (window.outerWidth === 0) {
            Object.defineProperty(window, 'outerWidth', { get: () => window.innerWidth });
        }
        if (window.outerHeight === 0) {
            Object.defineProperty(window, 'outerHeight', { get: () => window.innerHeight });
        }
    } catch (e) {}
})();

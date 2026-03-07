// Advanced Stealth Script (macOS/Safari 18.x Spoofing)
(function() {
    // Helper to overwrite read-only properties
    function mockReadonly(target, prop, value) {
        Object.defineProperty(target, prop, {
            get: () => value,
            configurable: true,
            enumerable: true
        });
    }

    // 1. Vendor Spoofing
    mockReadonly(navigator, 'vendor', 'Apple Computer, Inc.');

    // 2. ProductSub Spoofing (Safari standard)
    mockReadonly(navigator, 'productSub', '20030107');

    // 3. MaxTouchPoints (Desktop = 0)
    mockReadonly(navigator, 'maxTouchPoints', 0);

    // 4. Hardware Concurrency (Ensure realistic core count)
    if (navigator.hardwareConcurrency === undefined || navigator.hardwareConcurrency === 0) {
        mockReadonly(navigator, 'hardwareConcurrency', 8); // Typical Apple Silicon
    }

    // 5. Canvas Fingerprinting Noise
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

    // 6. Mask navigator.webdriver
    mockReadonly(navigator, 'webdriver', undefined);

    // 7. Mock Plugins and MimeTypes for Safari
    const pluginsData = [
        {
            name: 'WebKit built-in PDF',
            filename: '',
            description: '',
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

    // 8. WebGL Vendor/Renderer masking (Apple GPU)
    try {
        const getParameter = WebGLRenderingContext.prototype.getParameter;
        WebGLRenderingContext.prototype.getParameter = function(parameter) {
            // UNMASKED_VENDOR_WEBGL
            if (parameter === 37445) {
                return 'Apple';
            }
            // UNMASKED_RENDERER_WEBGL
            if (parameter === 37446) {
                return 'Apple GPU';
            }
            return getParameter(parameter);
        };
    } catch (e) {
        console.error('Failed to mask WebGL', e);
    }

    // 9. Bind outer dimensions
    try {
        if (window.outerWidth === 0) {
            Object.defineProperty(window, 'outerWidth', { get: () => window.innerWidth });
        }
        if (window.outerHeight === 0) {
            Object.defineProperty(window, 'outerHeight', { get: () => window.innerHeight });
        }
    } catch (e) {}

    // Note: No window.chrome mock
    // Note: No navigator.userAgentData mock
    // Note: No Permissions API mock (Safari doesn't expose it the same way)
})();

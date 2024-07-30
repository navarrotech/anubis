// Copyright © 2024 Navarrotech

pub fn create_vite_config() -> String {
    String::from(
        "
import { defineConfig } from 'vite'

// Node.js
import path from 'path'

// Plugins
import react from '@vitejs/plugin-react-swc'
import tsconfigPaths from 'vite-tsconfig-paths' // https://www.npmjs.com/package/vite-tsconfig-paths
import svgr from 'vite-plugin-svgr' // https://www.npmjs.com/package/vite-plugin-svgr

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    // Absolute imports:
    tsconfigPaths(),
    // Preact language + JSX:
    react(),
    // Svgs:
    svgr()
  ],
  css: {
    // Auto inject @use rules into headers of all scss/sass files before compiled
    // This only affects Vite css modules (*.module.sass or *.module.scss), and this does not global stylesheets
    // Great for using theme variables in all modules without having to re-import each time
    preprocessorOptions: {
      sass: {
        additionalData: `
          @use '@/sass/theme.sass' as *
          @use 'sass:color'
        `
      },
      scss: {
        additionalData: `
          @use '@/sass/theme.sass' as *;
          @use 'sass:color';
        `
      }
    }
  },
  resolve: {
    // Resolve all paths that start with @ to the root src/ directory:
    alias: {
      '@': path.resolve(__dirname, 'src')
    }
  }
})

",
    )
}

pub fn create_vitest_config() -> String {
    String::from(
        "
import { defineConfig } from 'vitest/config'

// For more information regarding this configuration:
// https://vitest.dev/config/

export default defineConfig({
  test: {
    // Reporting:
    reporters: [
      'junit'
    ],
    outputFile: {
      junit: './test/test-results.xml'
    },
    passWithNoTests: true,
  
    // Coverage (V8)
    coverage: {
      reporter: [
        'text-summary'
      ],
      reportsDirectory: './test/coverage',
      provider: 'v8'
    },
  
    // Typescript
    typecheck: {
      enabled: true
    },
  
    // React.js:
    globals: true,
    environment: 'happy-dom',
  
    // Circle CI:
    minWorkers: 2,
    maxWorkers: 3,
    logHeapUsage: true,
  
    // Debugging:
    onStackTrace(error, { file }): boolean | void {
      // If we've encountered a ReferenceError, show the whole stack.
      if (error.name === 'ReferenceError'){
        return
      }
  
      // Reject all frames from third party libraries.
      if (file.includes('node_modules')){
        return false
      }
    }
  }
})
",
    )
}

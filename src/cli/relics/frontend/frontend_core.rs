// Copyright © 2024 Navarrotech

use crate::automatrons::write::write_automatron;
use crate::relics::write::write_relic;
use crate::schema::AnubisSchema;

pub fn generate_frontend_core(schema: &AnubisSchema) {
    generate_env(schema);
    generate_vite_env(schema);
    generate_root_tsx(schema);
    generate_root_redux_store(schema);
}

fn generate_env(schema: &AnubisSchema) {
    let content = format!(
        r#"
export const NODE_ENV = import.meta.env.NODE_ENV || 'development'
export const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'
export const WEBSOCKET_URL = import.meta.env.VITE_GATEWAY_URL || 'ws://localhost:3000'

console.log('Running in ' + NODE_ENV + ' mode')
"#
    );

    write_relic(
        schema,
        &content,
        &schema.install_directory.join("frontend/src/env.ts"),
    );
}

fn generate_vite_env(schema: &AnubisSchema) {
    let content = format!(
        r#"/// <reference types="vite/client" />
/// <reference types="vite-plugin-svgr/client" />
"#
    );

    write_relic(
        schema,
        &content,
        &schema.install_directory.join("frontend/src/vite-env.d.ts"),
    );
}

fn generate_root_tsx(schema: &AnubisSchema) {
    let content = format!(
        r#"
// React.js
import {{ createRoot }} from 'react-dom/client'

// Application
import {{ Initialization }} from './store/Initialization'
import {{ ApplicationRouter }} from './routes/Router'

// Redux
import {{ Provider as ReduxProvider }} from 'react-redux'
import {{ store }} from './store/store'

// i18n
import '@/modules/i18n'

// Core CSS packages
import './index.sass'

const container = document.getElementById('root') as HTMLElement
const root = createRoot(container)

root.render(
  <ReduxProvider store={{store}}>
    <Initialization>
      <ApplicationRouter />
    </Initialization>
  </ReduxProvider>
)
"#
    );

    write_relic(
        schema,
        &content,
        &schema.install_directory.join("frontend/src/main.tsx"),
    );
}

fn generate_root_redux_store(schema: &AnubisSchema) {
    write_relic(
        schema,
        &format!(
            r#"
import {{
  type TypedUseSelectorHook,
  useDispatch as useDefaultDispatch,
  useSelector as useDefaultSelector
}} from 'react-redux'
import type {{ RootState, AppDispatch }} from './store'

type DispatchFunc = () => AppDispatch

export const useDispatch: DispatchFunc = useDefaultDispatch
export const useSelector: TypedUseSelectorHook<RootState> = useDefaultSelector

export {{ dispatch, getState }} from './store'
export type {{ AppDispatch, Thunk }} from './store'
"#
        ),
        &schema.install_directory.join("frontend/src/store/index.ts"),
    );

    write_relic(
        schema,
        &format!(
            r#"
// Add your own custom reducers here
export const customReducerSlices = {{
  // For example:
  // data: dataSlice.reducer,
}}
"#
        ),
        &schema
            .install_directory
            .join("frontend/src/store/custom.ts"),
    );

    // import { slice as userSlice } from '@/modules/auth/reducer'
    let mut reducer_imports = String::from("");
    // user: userSlice.reducer,
    let mut reducer_map = String::from("");

    for (i, model) in schema.models.iter().enumerate() {
        let slice = format!("{model_name}Slice", model_name = model.name.to_lowercase());
        let reducer = format!(
            "{model_name}: {slice}.reducer",
            model_name = model.name.to_lowercase(),
            slice = slice
        );
        let import = format!(
            "import {{ slice as {slice} }} from '@/modules/{model_name}/reducer'\n",
            slice = slice,
            model_name = model.name.to_lowercase()
        );

        if i != 0 {
            reducer_imports.push_str("\n");
            reducer_map.push_str("\n");
        }

        reducer_imports.push_str(&import);
        reducer_map.push_str(&reducer);
    }

    let content = format!(
        r#"
// Store configuration
import {{ type ThunkAction, configureStore, Action }} from '@reduxjs/toolkit'

// Reducers
import {{ customReducerSlices }} from './custom'
{reducer_imports}

// Environment
import {{ NODE_ENV }} from '@/env'

export const store = configureStore({{
  reducer: {{
    ...customReducerSlices,
    {reducer_map}
  }},
  middleware: getDefaultMiddleware =>
    getDefaultMiddleware({{
      thunk: true,
      serializableCheck: false
    }}),
  devTools: NODE_ENV === 'development'
}})

export const dispatch = store.dispatch
export const getState = store.getState

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>
// Inferred dispatch with everything we need!
export type AppDispatch = typeof store.dispatch

export type Thunk = ThunkAction<void, RootState, unknown, Action>
"#
    );

    write_automatron(
        schema,
        &content,
        &schema.install_directory.join("frontend/src/store/store.ts"),
    );
}

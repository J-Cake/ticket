import React from 'react';
import DOM from 'react-dom/client';

import AuthenticationManager from "./AuthenticationManager.js";

import '../css/main.css';
export {default as config} from '../config.json' with { type: 'json' };

export const root = DOM.createRoot(document.querySelector('#root')!);

export const authentication = new AuthenticationManager();

root.render(<authentication.IsAuthenticated>
	<App />
</authentication.IsAuthenticated>);

export function App() {
	return <div>{`Hello ${authentication.display}`}</div>
}
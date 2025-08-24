import React from 'react';
import DOM from 'react-dom/client';

import '../css/main.css';

export const root = DOM.createRoot(document.querySelector('#root')!);

root.render(<h1>{"Hello World"}</h1>);

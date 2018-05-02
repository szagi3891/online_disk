//@flow
import 'isomorphic-fetch';
import * as React from 'react';
import ReactDOM from 'react-dom';

import { Store } from './Store';
import { App } from './View/App';

const root = document.getElementById('root');

if (root) {
    const store = new Store();

    ReactDOM.render(
        <React.Fragment>
            <App store={store} />
        </React.Fragment>
        ,
        root
    );
} else {
    console.error('Error start app');
}

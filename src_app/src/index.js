//@flow
import 'isomorphic-fetch';
import * as React from 'react';
import ReactDOM from 'react-dom';

import { App } from './View/App';

//root

const root = document.getElementById('root');

if (root) {
    //const store = new Store(playlisty, findChannel());

    //runAudioReaction(store);

    ReactDOM.render(
        <React.Fragment>
            <App />
        </React.Fragment>
        ,
        root
    );
} else {
    console.error('Error start app');
}

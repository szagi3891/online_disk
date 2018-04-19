//@flow

import * as React from 'react';
import { observable } from 'mobx';
import { observer } from 'mobx-react';

@observer
export class App extends React.Component<PropsType> {
    @observable _counter: number;

    constructor(props: PropsType) {
        super(props);

        this._counter = 1;

        setInterval(() => {
            this._counter = this._counter + 1;
        }, 1000);
    }

    render(): React.Node {
        return (
            <div>
                To jest główny komponent { this._counter } ...
            </div>
        );
    }
}

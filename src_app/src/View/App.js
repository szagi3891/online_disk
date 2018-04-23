//@flow

import * as React from 'react';
import { observable } from 'mobx';
import { observer } from 'mobx-react';

import { Store } from '../Store';
const store = new Store();

type PropsType = {|
|};

@observer
export class App extends React.Component<PropsType> {
    @observable _counter: number;

    @observable input_folder: string;

    constructor(props: PropsType) {
        super(props);

        this._counter = 1;
        this.input_folder = '';

        setInterval(() => {
            this._counter = this._counter + 1;
        }, 1000);
    }

    render(): React.Node {
        return (
            <React.Fragment>
                <div>
                    To jest główny komponent { this._counter } ...
                    <button onClick={this._getHead}>Pobierz heada</button>
                </div>
                <div>
                    { this._renderHead() }
                </div>
                <div>
                    <input value={this.input_folder} onChange={this._onChangeInput} />
                    <button onClick={this._onClickDodaj}>Dodaj</button>
                </div>
            </React.Fragment>
        );
    }

    _getHead = () => {
        store.getHead();
    }

    _renderHead(): React.Node {
        const head = store.head;
        if (head === null) {
            return '---';
        }

        return (
            <div>
                <div>{ head.head }</div>
                <div>{ head.counter }</div>
            </div>
        );
    }

    _onChangeInput = (event: SyntheticEvent<>) => {
        const { target } = event;
        if (target instanceof HTMLInputElement) {
            this.input_folder = target.value;
        }
    }

    _onClickDodaj = () => {
        console.info("Zaczynam dodawać", this.input_folder);
        store.addDir(this.input_folder).then(() => {
            console.info("Koniec dodawania");
        });
    }
}

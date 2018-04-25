//@flow

import * as React from 'react';
import { observable } from 'mobx';
import { observer } from 'mobx-react';
import { Store } from '../Store';

type PropsType = {|
    store: Store
|};

@observer
export class DirAdd extends React.Component<PropsType> {
    @observable input_folder: string;

    constructor(props: PropsType) {
        super(props);

        this.input_folder = '';
    }

    render(): React.Node {
        return (
            <div>
                <input value={this.input_folder} onChange={this._onChangeInput} />
                <button onClick={this._onClickDodaj}>Dodaj</button>
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
        this.props.store.dir.add(this.input_folder).then(() => {
            console.info("Koniec dodawania");
        });
        this.input_folder = '';
    }
}
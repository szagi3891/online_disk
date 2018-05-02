//@flow

import * as React from 'react';
import { observable } from 'mobx';
import { observer } from 'mobx-react';
import { Store } from '../Store';
import { DirItem } from '../Store/Root/DirItem';

type PropsType = {|
    store: Store,
    dirItem: DirItem
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
                <button onClick={this._onClickAddDir}>Dodaj katalog</button>
            </div>
        );
    }

    _onChangeInput = (event: SyntheticEvent<>) => {
        const { target } = event;
        if (target instanceof HTMLInputElement) {
            this.input_folder = target.value;
        }
    }

    _onClickAddDir = () => {
        const { dirItem } = this.props;

        console.info('Zaczynam dodawać', this.input_folder);
        dirItem.addDir(this.input_folder).then(() => {
            console.info('Koniec dodawania');
        }).catch((error: mixed) => {
            console.error('Otrzymano błąd', error);
        });

        this.input_folder = '';
    }
}
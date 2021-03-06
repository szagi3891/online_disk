import * as React from 'react';
import { observable } from 'mobx';
import { observer } from 'mobx-react';
import { Store } from 'Store';
import { DirItem } from 'Store/Root/DirItem';

interface PropsType {
    store: Store,
    dirItem: DirItem
}

@observer
export class FileAddEmpty extends React.Component<PropsType> {
    @observable input_file: string;

    constructor(props: PropsType) {
        super(props);

        this.input_file = '';
    }

    render() {
        return (
            <div>
                <input value={this.input_file} onChange={this._onChangeInput} />
                <button onClick={this._onClickAddDir}>Dodaj pusty plik tekstowy</button>
            </div>
        );
    }

    _onChangeInput = (event: React.SyntheticEvent) => {
        const { target } = event;
        if (target instanceof HTMLInputElement) {
            this.input_file = target.value;
        }
    }

    _onClickAddDir = () => {
        const { dirItem } = this.props;

        const newFileName = `${this.input_file}.txt`;

        dirItem.addEmptyFile(newFileName).then(() => {
            console.info('Koniec dodawania');
        }).catch((error: any) => {
            console.error('Otrzymano błąd', error);
        });

        this.input_file = '';
    }
}
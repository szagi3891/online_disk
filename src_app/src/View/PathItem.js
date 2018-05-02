//@flow

import * as React from 'react';
import { List as IList } from 'immutable';
import { observer } from 'mobx-react';
import glamorous from 'glamorous';
import { Store } from '../Store';
import rgba from 'hex-rgba';
import { DirListItemName } from './DirListItemName';

const PathItemBox = glamorous.span({
    paddingRight: '5px'
});

const PathItemNoActive = glamorous(PathItemBox)({
    cursror: 'default'
});

const PathItemSpanClick = glamorous(PathItemBox)({
    cursor: 'pointer',
    color: rgba('#0000ff', 50),
    ':hover': {
        color: rgba('#0000ff', 25)
    }
});

type PropsType = {|
    store: Store,
    caption: string,
    path: IList<string>,
    idDir: bool
|};

@observer
export class PathItem extends React.Component<PropsType> {
    render(): React.Node {
        const { path, caption, idDir, store } = this.props;

        const hasCurrentPath = store.path.hasCurrentSet(path);

        if (hasCurrentPath) {
            return (
                <PathItemNoActive>
                    <DirListItemName name={caption} isDir={idDir} />
                </PathItemNoActive>
            );
        }

        return (
            <React.Fragment>
                <PathItemSpanClick onClick={this._onClick}>
                    <DirListItemName name={caption} isDir={idDir} />
                </PathItemSpanClick>
                <PathItemBox>&gt;</PathItemBox>
            </React.Fragment>
        );
    }

    _onClick = () => {
        const { store, path } = this.props;
        store.path.goTo(path);
    }
}

import * as React from 'react';
import { List as IList } from 'immutable';
import { observer } from 'mobx-react';
import styled from 'react-emotion';
import { Store } from 'Store';
//@ts-ignore TODO
import rgba from 'hex-rgba';
import { DirListItemName } from 'View/DirListItemName';

const PathItemBox = styled('span')`
    padding-right: 5px;
`;

const PathItemNoActive = styled(PathItemBox)`
    cursor: default;
`;

const PathItemSpanClick = styled(PathItemBox)`
    cursor: pointer;
    color: ${rgba('#0000ff', 50)};
    &:hover: {
        color: ${rgba('#0000ff', 25)};
    }
`;

type PropsType = {
    store: Store,
    caption: string,
    path: IList<string>,
    idDir: boolean
}

@observer
export class PathItem extends React.Component<PropsType> {
    render() {
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

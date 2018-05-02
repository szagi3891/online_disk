//@flow

import * as React from 'react';
import { observer } from 'mobx-react';
import glamorous from 'glamorous';
//import rgba from 'hex-rgba';
import { DirIcon } from './Icon/DirIcon';
import { FileIcon } from './Icon/FileIcon';
import { css } from 'glamor';

const Main = glamorous.div({
    display: 'flex'
});

const DivMain = glamorous.div({
    flexShrink: '0'
});

const dirClass = css({
    flexShrink: '0',
    width: '20px',
    height: '20px',
    marginRight: '5px'
});

const renderIcon = (isDir: bool) => {
    if (isDir) {
        return (
            <DirIcon className={dirClass} />
        );
    }

    return (
        <FileIcon className={dirClass} />
    );
};

type PropsType = {|
    name: string,
    isDir: bool,
|};

@observer
export class DirListItemName extends React.Component<PropsType> {
    render() {
        const { isDir, name } = this.props;

        return (
            <Main>
                { renderIcon(isDir) }
                <DivMain>{name}</DivMain>
            </Main>
        );        
    }
}

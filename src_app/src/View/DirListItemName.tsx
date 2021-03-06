import * as React from 'react';
import { observer } from 'mobx-react';
import { css } from 'emotion';
import styled from 'react-emotion';
import { DirIcon } from 'View/Icon/DirIcon';
import { FileIcon } from 'View/Icon/FileIcon';

const Main = styled('div')`
    display: flex;
`;

const DivMain = styled('div')`
    flex-shrink: 0;
`;

const dirClass = css`
    flex-shrink: 0;
    width: 20px;
    height: 20px;
    margin-right: 5px;
`;

const renderIcon = (isDir: boolean) => {
    if (isDir) {
        return (
            <DirIcon className={dirClass} />
        );
    }

    return (
        <FileIcon className={dirClass} />
    );
};

interface PropsType {
    name: string,
    isDir: boolean,
}

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

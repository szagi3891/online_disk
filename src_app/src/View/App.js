//@flow

import * as React from 'react';
import { observable } from 'mobx';
import { observer } from 'mobx-react';
import { css } from 'glamor';
import glamorous from 'glamorous';

import { Store } from '../Store';
import { DirAddEmpty } from './Add/DirAddEmpty';
import { FileAddEmpty } from './Add/FileAddEmpty';
import { DirList } from './DirList';
import { Path } from './Path';

const AppWrapper = glamorous.div({
    display: 'flex',
    flexDirection: 'column',
    minHeight: '100vh'
});

const pathClassName = css({
    borderBottom: '1px solid black',
    padding: '5px'
});

const MainContentWrapper = glamorous.div({
    display: 'flex',
    overflow: 'hidden',
    flexGrow: '1'
});

const dirListClassName = css({
    flexGrow: '1',
    maxWidth: '400px',
    borderRight: '1px solid black'
});

const ContentWrapper = glamorous.div({
    flexGrow: '1',
    flexShrink: '0',
    marginLeft: '5px'
});

const OptionWrapper = glamorous.div({
    display: 'flex',
    justifyContent: 'center',
    position: 'absolute',
    width: '70px',
    top: '0',
    right: '0',
    border: '1px solid black',
    padding: '5px',
    cursor: 'pointer',
    backgroundColor: 'white',
    ':hover': {
        backgroundColor: '#e0e0e0'
    }
});

const OptionBody = glamorous.div({
    borderBottom: '1px solid black',
    padding: '5px'
});

type PropsType = {|
    store: Store,
|};

@observer
export class App extends React.Component<PropsType> {
    @observable _showFlag: bool = false;

    render(): React.Node {
        const { store } = this.props;
        return (
            <AppWrapper>
                <Path className={pathClassName} store={store} />
                { this._renderOptionBody() }
                <MainContentWrapper>
                    { this._renderDirList() }
                    { this._renderContent() }
                </MainContentWrapper>
                { this._renderOptionButton() }
            </AppWrapper>
        );
    }

    _renderAddDir() {
        const { store } = this.props;
        const pathDir = store.root.currentPathNodes.path;
        const last = pathDir.last();

        if (last) {
            return (
                <DirAddEmpty store={store} dirItem={last} />
            );
        }

        return null;
    }

    _renderAddEmptyTextFile() {
        const { store } = this.props;
        const pathDir = store.root.currentPathNodes.path;
        const last = pathDir.last();

        if (last) {
            return (
                <FileAddEmpty store={store} dirItem={last} />
            );
        }

        return null;
    }

    _renderDirList() {
        const { store } = this.props;
        const pathDir = store.root.currentPathNodes.path;
        const last = pathDir.last();

        if (last) {
            return (
                <DirList className={dirListClassName} store={store} dirItem={last} />
            );
        }

        return (
            <div>Ładowanie listy ...</div>
        );
    }

    _renderHead(): React.Node {
        const { store } = this.props;
        const head = store.head;
        if (head === null) {
            return '---';
        }

        return (
            <div>{ head.head } - { head.counter }</div>
        );
    }

    _renderContent(): React.Node {
        return (
            <ContentWrapper>
                dasdas
            </ContentWrapper>
        );
    }

    _hideOption = () => {
        this._showFlag = false;
    }

    _showOption = () => {
        this._showFlag = true;
    }

    _renderOptionButton(): React.Node {
        if (this._showFlag) {
            return (
                <OptionWrapper>
                    <div onClick={this._hideOption}>Hide</div>
                </OptionWrapper>
            );
        }
    
        return (
            <OptionWrapper>
                <div onClick={this._showOption}>Show</div>
            </OptionWrapper>
        );
    }

    _renderOptionBody() {
        if (this._showFlag) {
            return (
                <OptionBody>
                    { this._renderHead() }
                    { this._renderAddDir() }
                    { this._renderAddEmptyTextFile() }
                </OptionBody>
            );
        }

        return null;
    }
}

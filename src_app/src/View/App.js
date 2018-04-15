//@flow

import * as React from 'react';
import { observer } from 'mobx-react';

@observer
export class App extends React.Component<PropsType> {
    render(): React.Node {
        return (
            <div>
                To jest główny komponent
            </div>
        );
    }
}

const path = require('path');
const webpack = require('webpack');

const getNodeConfig = () => ({
    target: 'node',
    node: {
        __filename: true
    }
});

const make = (mode, entryFileName, outFileName) => {
    const nodeConf = mode === 'client' ? {} : getNodeConfig();

    return {
        mode: 'development',
        //mode: 'production',
        entry: entryFileName,
        output: {
            path: path.join(__dirname, './dist/static'),
            publicPath: '/static/',
            filename: outFileName,
            pathinfo: true
        },
        module: {
            rules: [
                {
                    test: /\.(svg|png|jpg|gif|ico)$/,
                    loaders: ['file-loader'],
                },
                { test: /\.tsx?$/, loader: "awesome-typescript-loader" },
            ],
        },
        resolve: {
            extensions: ['.ts', '.tsx', '.js'],
            modules: [
                path.join(__dirname, 'src'),
                'node_modules'
            ]
        },
        target: nodeConf.target,
        node: nodeConf.node
    };
};

module.exports = [
    make('client', './src/index.tsx', 'index.js')
];

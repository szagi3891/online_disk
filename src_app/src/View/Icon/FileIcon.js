//@flow
import * as React from 'react';

type PropsType = {|
    className: string,
|};

export class FileIcon extends React.Component<PropsType> {
    render() {
        const { className } = this.props;

        return (
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" className={className}>
                <defs>
                    <linearGradient id="0" x1="452.11" y1="556.82" x2="451.76" y2="528.82" gradientUnits="userSpaceOnUse">
                    <stop stopColor="#197cf1"/>
                    <stop offset="1" stopColor="#20bcfa"/>
                    </linearGradient>
                </defs>
                <g transform="translate(-384.57-515.8)">
                    <rect width="22.411" height="29.671" x="440.55" y="528.02" fill="url(#0)" fillRule="evenodd" stroke="#0b3969" strokeLinecap="round" strokeWidth=".3" rx="1" transform="translate(-51.19-11.03)"/>
                    <path d="m394.57 535.13v2.673h2.673l6.751-6.751-2.673-2.673zm11.788-6.434c.283-.283.283-.728 0-1.01l-1.671-1.671c-.283-.283-.728-.283-1.01 0l-1.396 1.405 2.673 2.673z" fill="#fff" stroke="none"/>
                </g>
            </svg>
        );
    }
}

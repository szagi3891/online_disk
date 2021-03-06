import * as React from 'react';

interface PropsType {
    className: string,
}

export class DirIcon extends React.Component<PropsType> {
    render() {
        const { className } = this.props;

        return (
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" className={className}>
                <defs>
                    <linearGradient
                        id="0"
                        x1="59.12"
                        y1="-19.888"
                        x2="59.15"
                        y2="-37.783"
                        gradientUnits="userSpaceOnUse"
                        gradientTransform="matrix(4.17478 0 0 4.16765-1069.7 447.73)"
                    >
                        <stop stopColor="#6992e2" />
                        <stop offset="1" stopColor="#84b0ff"/>
                    </linearGradient>
                    <linearGradient gradientUnits="userSpaceOnUse" y2="354.29" x2="-704.05" y1="647.77" x1="-701.19" id="1"
                        ><stop stopColor="#7ba2fd"/>
                        <stop offset="1" stopColor="#aec7ff"/>
                    </linearGradient>
                    <linearGradient gradientUnits="userSpaceOnUse" y2="352.98" x2="-601.15" y1="663.95" x1="-591.02" id="2">
                        <stop stopColor="#729cec"/>
                        <stop offset="1" stopColor="#75a0ef"/>
                    </linearGradient>
                </defs>
                <g transform="matrix(.07089 0 0 .07017 23.295-40.67)" fill="#60aae5">
                    <path d="m-884.1 294.78c-4.626 0-8.349 3.718-8.349 8.335v161.41l468.19 1v-121.2c0-4.618-3.724-8.335-8.35-8.335h-272.65c-8.51.751-9.607-.377-13.812-5.981-5.964-7.968-14.969-21.443-20.84-29.21-4.712-6.805-5.477-6.02-13.292-6.02z" transform="matrix(.7872 0 0 .79524 415.34 430.11)" fill="url(#0)" color="#000"/>
                    <rect width="463.85" height="295.13" x="-890.28" y="356.85" transform="matrix(.7872 0 0 .79524 415.34 430.11)" fill="url(#1)" stroke="url(#1)" strokeWidth="2.378" rx="9.63"/>
                    <rect width="463.85" height="295.13" x="-890.28" y="356.85" transform="matrix(.7872 0 0 .79524 415.34 430.11)" fill="none" stroke="url(#2)" strokeLinejoin="round" strokeLinecap="round" strokeWidth="5.376" rx="9.63"/>
                </g>
            </svg>
        );
    }
}
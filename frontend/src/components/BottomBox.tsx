import React from 'react';

interface BottomBoxProps extends React.HTMLProps<HTMLDivElement> {
    children: React.ReactNode;
}

const BottomBox: React.FC<BottomBoxProps> = ({ children, ...props }) => {
    return (
        <div className="bg-black/40 rounded-xl backdrop-blur-sm border border-white/10 p-4">
            <div className="flex justify-start gap-6" {...props}>
                {children}
            </div>
        </div>
    );
};

export default BottomBox;
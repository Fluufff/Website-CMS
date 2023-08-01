import { FC, useEffect, useState } from 'react';
import cx from 'classnames/bind';
import ReactModal from 'react-modal';

import { Button, ButtonSizes, ButtonTypes } from '../button';

import styles from './modal.module.scss';
import { IModalFooterProps, IModalProps } from './modal.types';

const cxBind = cx.bind(styles);

export const Modal: FC<IModalProps> = ({ title, children, modalOpen }: IModalProps) => {
	const [isOpen, setIsOpen] = useState(true);

	useEffect(() => {
		setIsOpen(modalOpen);
	}, [modalOpen]);

	return (
		<ReactModal isOpen={isOpen} className={cxBind('m-modal')} onRequestClose={() => setIsOpen(false)}>
			<div className={cxBind('m-modal__header')}>
				<h2 className={cxBind('m-modal__title')}>{title}</h2>
				<Button onClick={() => setIsOpen(false)} size={ButtonSizes.SMALL} type={ButtonTypes.TRANSPARENT} className={cxBind('m-modal__close')}>
					<span className="las la-times"></span>
				</Button>
			</div>
			{children}
		</ReactModal>
	);
};

export const ModalFooter: FC<IModalFooterProps> = ({ children }: IModalFooterProps) => {
	return <div className={cxBind('m-modal__footer')}>{children}</div>
}

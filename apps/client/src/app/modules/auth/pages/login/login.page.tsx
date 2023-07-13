import { Alert, AlertTypes, Button, ButtonTypes, HTMLButtonTypes, TextInput, TextInputTypes } from "@ibs/components"
import cx from 'classnames/bind';
import classNames from "classnames";
import { FormProvider, useForm } from "react-hook-form";
import { yupResolver } from "@hookform/resolvers/yup"

import { useAuthStore } from "../../../core/stores/auth.store";
import { IErrorResponse } from "../../../core/types/error.types";

import styles from './login.module.scss';
import { loginSchema } from "./login.const";
const cxBind = cx.bind(styles);

interface ILoginFormInput {
	email: string;
	password: string;
}

export const LoginPage = () => {
	const authStore = useAuthStore();
	const formMethods = useForm<ILoginFormInput>({ resolver: yupResolver(loginSchema) });
	const { handleSubmit, setError, formState: { errors } } = formMethods;

	const onSubmit = ({ email, password }: ILoginFormInput) => {
		authStore.login(email, password)
			.catch((error: IErrorResponse) => {
				setError('root', {
					message: error.code
				})
			});
	}

	return (
		<div className={cxBind('p-login')}>
			<div className={cxBind('p-login__content')}>
				<div className={cxBind('p-login__form')}>
					<div className={classNames("u-margin-bottom-lg", cxBind('p-login__logo'))}>
						<i className="las la-sms"></i> Inhoud Beheer Systeem
					</div>
					<Alert className="u-margin-bottom" type={AlertTypes.DANGER}>{errors?.root?.message}</Alert>
					<FormProvider {...formMethods}>
						<form onSubmit={handleSubmit(onSubmit)}>
							<div className="u-margin-bottom">
								<TextInput name="email" label="Email" type={TextInputTypes.EMAIL} options={{ required: true }} />
							</div>
							<div className="u-margin-bottom">
								<TextInput name="password" label="Password" type={TextInputTypes.PASSWORD} options={{ required: true }} />
							</div>
							<div>
								<Button className="u-margin-right-sm" htmlType={HTMLButtonTypes.SUBMIT}>Login</Button>
								<Button type={ButtonTypes.TRANSPARENT}>Forgot password</Button>
							</div>
						</form>
					</FormProvider>
				</div>
			</div>
			<div className={cxBind('p-login__aside')} style={{
				backgroundImage: `url(https://images.unsplash.com/photo-1563923683738-4ad77b43411c?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D)`
			}}></div>
		</div>
	)
}

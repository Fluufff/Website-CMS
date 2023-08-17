import { IAPIError, useAuthenticationMethodStore } from '@ibs/shared';
import { useParams } from 'react-router-dom';
import { Alert, AlertTypes, Button, ButtonTypes, HTMLButtonTypes } from '@ibs/components';
import { FormProvider, useForm } from 'react-hook-form';
import { yupResolver } from '@hookform/resolvers/yup';
import { NumberField, SelectField, TextField, ToggleField } from '@ibs/forms';

import { AUTHENTICATION_METHOD_OPTIONS } from '../../authentication-methods.const';

import { editAuthenticationMethodSchema } from './authentication-method-info.const';

interface EditAuthenticationMethodForm {
	name: string;
	active: boolean;
	weight: number;
}

export const AuthenticationMethodInfoPage = () => {
	const [authenticationMethod] = useAuthenticationMethodStore((state) => [
		state.authenticationMethod,
	]);
	const [updateAuthenticationMethodLoading, updateAuthenticationMethod] = useAuthenticationMethodStore((state) => [
		state.updateAuthenticationMethodLoading,
		state.updateAuthenticationMethod,
	]);
	const { authenticationMethodId } = useParams();
	const formMethods = useForm<EditAuthenticationMethodForm>({
		resolver: yupResolver(editAuthenticationMethodSchema),
		values: authenticationMethod,
	});

	const {
		handleSubmit,
		formState: { errors },
		setError,
	} = formMethods;

	const onSubmit = (values: EditAuthenticationMethodForm) => {
		if (!authenticationMethodId) {
			return;
		}

		updateAuthenticationMethod(authenticationMethodId, {
			...authenticationMethod,
			...values,
		}).catch((error: IAPIError) => {
			setError('root', {
				message: error.code,
			});
		});
	};

	return (
		<FormProvider {...formMethods}>
			<Alert className="u-margin-bottom" type={AlertTypes.DANGER}>
				{errors?.root?.message}
			</Alert>
			<form onSubmit={handleSubmit(onSubmit)}>
				<div className="u-margin-bottom">
					<TextField name="name" label="Name" />
				</div>
				<div className="u-margin-bottom">
					<NumberField name="weight" label="Weight" />
				</div>
				<div className="u-margin-bottom">
					<ToggleField name="active" label="Active" />
				</div>
				<div className="u-margin-bottom">
					<SelectField name="kind" label="Kind" disabled fieldConfiguration={{ options: AUTHENTICATION_METHOD_OPTIONS }} />
				</div>
				<Button type={ButtonTypes.PRIMARY} htmlType={HTMLButtonTypes.SUBMIT}>
					{updateAuthenticationMethodLoading && <i className="las la-redo-alt la-spin"></i>} Save
				</Button>
			</form>
		</FormProvider>
	);
};
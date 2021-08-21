/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   rate_calc.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/08/21 14:12:54 by vdragomi          #+#    #+#             */
/*   Updated: 2021/08/21 22:05:26 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

//the formula for interest rate is:
//r = n[(A / P) ^ 1 / (n * t) - 1]
//Note that you should multiply your result by 100 to get a percentage figure (%)

pub fn rate_calc(principal:f64, final:f64, time:f64, pmt:f64, n:f64) => f64
{
	let power = f64:powf()
}
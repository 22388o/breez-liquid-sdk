#import <React/RCTBridgeModule.h>
#import <React/RCTEventEmitter.h>

@interface RCT_EXTERN_MODULE(RNBreezLiquidSDK, RCTEventEmitter)
{% for func in ci.function_definitions() %}
{%- if func.name()|ignored_function == false -%}
{% include "ExternFunctionTemplate.m" %}
{% endif %}
{%- endfor %}  
RCT_EXTERN_METHOD(
    connect: (NSString*)mnemonic
    dataDir: (NSString*)dataDir
    network: (NSString*)network
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)
{%- for type_ in ci.iter_types() %}
{%- let type_name = type_|type_name %}
{%- match type_ %}
{%- when Type::Object ( name ) %}
{% let obj = ci.get_object_definition(name).unwrap() %}
{%- for func in obj.methods() -%}
{%- include "ExternFunctionTemplate.m" %}
{% endfor %}
{%- else -%}
{%- endmatch -%}    
{%- endfor %}
@end